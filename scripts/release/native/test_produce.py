"""Synthetic fixtures only: no compiler, native build, graphics or network runs."""
import hashlib
import json
import os
import tempfile
import unittest
from pathlib import Path
from produce import EvidenceError, collect, verify_replay

class FakeRunner:
    def __init__(self, repo, sysroot, source, tree, target, reference):
        self.repo=repo;self.sysroot=sysroot;self.source=source;self.tree=tree;self.target=target;self.reference=reference;self.package='ratatui'
        self.calls=[];self.dirty=False;self.zero_tests=False;self.bad_replay=False;self.mutate_binary=False;self.fail_build=False;self.bad_test_path=False
    def __call__(self, argv, cwd, env, timeout):
        self.calls.append((list(argv),dict(env)))
        if argv[:3]==['git','rev-parse','--show-toplevel']: return str(self.repo)+'\n',''
        if argv[:3]==['git','rev-parse','HEAD']: return self.source+'\n',''
        if argv[:3]==['git','rev-parse','HEAD^{tree}']: return self.tree+'\n',''
        if argv[:2]==['git','status']: return ('?? unknown\n' if self.dirty else ''),''
        if argv==['rustc','--print','sysroot']: return str(self.sysroot)+'\n',''
        if Path(argv[0]).name in ('rustc','rustc.exe') and argv[1:] == ['-vV']: return 'rustc 1.99.1\ncommit-hash: '+'b'*40+'\nhost: '+self.target+'\n',''
        if Path(argv[0]).name in ('cargo','cargo.exe') and argv[1:] == ['--version']: return 'cargo fixture\n',''
        exe={'ratatui':'pong-ratatui','softbuffer':'pong-softbuffer','sdl3':'pong-sdl3'}[self.package]
        suffix='.exe' if self.target.endswith('-windows-msvc') else ''
        if Path(argv[0]).name in ('cargo','cargo.exe') and argv[1]=='build':
            if self.fail_build: raise EvidenceError('fixture build failed')
            binary=Path(env['CARGO_TARGET_DIR'])/self.target/'release'/(exe+suffix);binary.parent.mkdir(parents=True);binary.write_bytes(b'fresh fixture binary')
            return 'fixture build succeeded\n',''
        if Path(argv[0]).name in ('cargo','cargo.exe') and argv[1]=='test':
            testbin=(self.repo.parent if self.bad_test_path else Path(env['CARGO_TARGET_DIR'])/self.target/'release'/'deps')/('fixture-test'+suffix)
            testbin.parent.mkdir(parents=True,exist_ok=True);testbin.write_bytes(b'fixture test binary')
            return json.dumps({'reason':'compiler-artifact','profile':{'test':True},'target':{'name':exe,'kind':['bin']},'executable':str(testbin)})+'\n',''
        if argv[-1]=='--nocapture': return ('test result: ok. 0 passed;\n' if self.zero_tests else 'test result: ok. 15 passed;\n'),''
        if argv[-1]=='--scripted-test':
            if self.mutate_binary: Path(argv[0]).write_bytes(b'modified')
            return (self.reference.replace('"tick": 480','"tick": 481') if self.bad_replay else self.reference),''
        raise AssertionError(argv)

class ProducerTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();base=Path(self.tmp.name);self.repo=base/'checkout';self.repo.mkdir();self.out=base/'result';self.sysroot=base/'sysroot';self.target='aarch64-unknown-linux-gnu';self.source='a'*40;self.tree='c'*40
        (self.repo/'experiments/ratatui').mkdir(parents=True);(self.repo/'experiments/ratatui/Cargo.toml').write_text('fixture')
        (self.repo/'docs/evidence').mkdir(parents=True)
        self.reference=''.join(json.dumps({'scenario':'smoke-v1','seed':42,'tick':i*120,'state':'Ready','scores':[0,0],'finished':i==12,'player':250,'cpu':250,'ball':[450,250,0,0]})+'\n' for i in range(13))
        (self.repo/'docs/evidence/rust-core-contact-timing-model-20261112.jsonl').write_text(self.reference)
        (self.sysroot/'bin').mkdir(parents=True)
        for name in ('rustc','cargo','rustc.exe','cargo.exe'): (self.sysroot/'bin'/name).write_bytes(name.encode())
        libs=self.sysroot/'lib/rustlib'/self.target/'lib';libs.mkdir(parents=True);(libs/'libstd.rlib').write_bytes(b'std')
        self.runner=FakeRunner(self.repo,self.sysroot,self.source,self.tree,self.target,self.reference)
    def tearDown(self):self.tmp.cleanup()
    def collect(self): return collect(self.repo,self.out,'ratatui',self.target,self.source,self.tree,self.runner)
    def test_fresh_outputs_bind_actual_fixture_commands_and_evidence(self):
        record=self.collect();self.assertEqual(record['binary_sha256'],hashlib.sha256(b'fresh fixture binary').hexdigest());self.assertEqual(record['rustc_commit'],'b'*40)
        self.assertEqual(record['source_sha'],self.source);self.assertEqual(record['tree_sha'],self.tree);self.assertEqual(record['target'],self.target)
        self.assertIn('not a notice',record['limitations']);self.assertTrue((self.out/'build-evidence.json').exists())
        commands=[x[0] for x in self.runner.calls];build=next(x for x in commands if len(x)>1 and x[1]=='build');self.assertIn('--locked',build);self.assertIn('--offline',build);self.assertIn('--release',build);self.assertIn(self.target,build)
        env=next(e for a,e in self.runner.calls if len(a)>1 and a[1]=='build');self.assertEqual(os.path.normcase(os.path.normpath(env['RUSTC'])),os.path.normcase(os.path.normpath(str(self.sysroot/'bin/rustc'))));self.assertEqual(env['CARGO_NET_OFFLINE'],'true')
    def test_configured_wrappers_and_runner_cannot_execute_tests(self):
        from unittest.mock import patch
        config=self.repo/'.cargo';config.mkdir();(config/'config.toml').write_text('[build]\nrustc-wrapper="bad"\n[target.aarch64-unknown-linux-gnu]\nrunner="bad"\n')
        with patch.dict('os.environ',{'RUSTC_WRAPPER':'bad','RUSTC_WORKSPACE_WRAPPER':'bad','CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER':'bad'}): self.collect()
        test_cmd,env=next((a,e) for a,e in self.runner.calls if len(a)>1 and a[1]=='test')
        self.assertIn('--no-run',test_cmd);self.assertIn('--message-format=json',test_cmd)
        self.assertEqual(env['RUSTC_WRAPPER'],'');self.assertEqual(env['RUSTC_WORKSPACE_WRAPPER'],'')
        self.assertNotIn('CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER',env)
        direct=[a for a,e in self.runner.calls if a[-1]=='--nocapture'];self.assertEqual(len(direct),1);self.assertTrue(Path(direct[0][0]).name.startswith('fixture-test'))
    def test_windows_exe_command_plans(self):
        for package in ('ratatui','softbuffer'):
            with self.subTest(package=package):
                target='x86_64-pc-windows-msvc';self.target=target;self.runner.target=target;self.runner.package=package
                (self.repo/'experiments'/package).mkdir(parents=True,exist_ok=True);(self.repo/'experiments'/package/'Cargo.toml').write_text('fixture')
                libs=self.sysroot/'lib/rustlib'/target/'lib';libs.mkdir(parents=True,exist_ok=True);(libs/'libstd.rlib').write_bytes(b'std')
                self.out=self.repo.parent/('result-'+package)
                record=collect(self.repo,self.out,package,target,self.source,self.tree,self.runner)
                self.assertTrue(record['binary_relative_path'].endswith('.exe'))
                relevant=[a for a,e in self.runner.calls if a[-1] in ('--nocapture','--scripted-test')]
                self.assertTrue(all(a[0].endswith('.exe') for a in relevant[-2:]))
                self.assertTrue(any(Path(a[0]).name=='cargo.exe' and len(a)>1 and a[1]=='build' for a,e in self.runner.calls))
    def test_outside_test_binary_is_rejected(self):
        self.runner.bad_test_path=True
        with self.assertRaisesRegex(EvidenceError,'outside fresh target'):self.collect()
        self.assertFalse((self.out/'build-evidence.json').exists())
    def test_dirty_or_wrong_source_fails_before_output(self):
        self.runner.dirty=True
        with self.assertRaisesRegex(EvidenceError,'not clean'):self.collect()
        self.assertFalse(self.out.exists());self.runner.dirty=False;self.runner.source='d'*40
        with self.assertRaisesRegex(EvidenceError,'mismatch'):self.collect()
    def test_no_passing_tests_or_bad_replay_never_writes_success(self):
        self.runner.zero_tests=True
        with self.assertRaisesRegex(EvidenceError,'no passing'):self.collect()
        self.assertFalse((self.out/'build-evidence.json').exists())
        self.out=self.out.parent/'result2';self.runner.zero_tests=False;self.runner.bad_replay=True
        with self.assertRaisesRegex(EvidenceError,'mismatch'):self.collect()
        self.assertFalse((self.out/'build-evidence.json').exists())
    def test_build_failure_never_writes_success(self):
        self.runner.fail_build=True
        with self.assertRaisesRegex(EvidenceError,'build failed'):self.collect()
        self.assertFalse((self.out/'build-evidence.json').exists())
    def test_missing_sysroot_library_is_rejected(self):
        (self.sysroot/'lib/rustlib'/self.target/'lib'/'libstd.rlib').unlink()
        with self.assertRaisesRegex(EvidenceError,'empty target sysroot'):self.collect()
        self.assertFalse(self.out.exists())
    def test_changed_binary_is_rejected(self):
        self.runner.mutate_binary=True
        with self.assertRaisesRegex(EvidenceError,'changed'):self.collect()
        self.assertFalse((self.out/'build-evidence.json').exists())
    def test_fresh_output_must_be_outside_checkout_and_absent(self):
        self.out=self.repo/'nested'
        with self.assertRaisesRegex(EvidenceError,'outside'):self.collect()
        self.out=self.repo.parent/'existing';self.out.mkdir()
        with self.assertRaisesRegex(EvidenceError,'new'):self.collect()
    def test_incomplete_replay_and_nonfinite_numbers_rejected(self):
        with self.assertRaisesRegex(EvidenceError,'13 records'):verify_replay('\n'.join(self.reference.splitlines()[:-1]),self.reference)
        rows=[json.loads(x) for x in self.reference.splitlines()];rows[3]['ball'][0]=float('nan')
        with self.assertRaisesRegex(EvidenceError,'numeric'):verify_replay('\n'.join(json.dumps(x) for x in rows),self.reference)
    def test_cross_target_and_noninitial_pair_rejected(self):
        with self.assertRaisesRegex(EvidenceError,'scope'):collect(self.repo,self.out,'sdl3',self.target,self.source,self.tree,self.runner)
        self.runner.target='x86_64-pc-windows-msvc'
        with self.assertRaisesRegex(EvidenceError,'cross-target'):self.collect()

if __name__=='__main__':unittest.main()
