import json, os, tempfile, unittest
from pathlib import Path
from unittest.mock import patch
import handoff as h
import produce_then_handoff as bridge

class HandoffTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.addCleanup(self.tmp.cleanup);self.root=Path(self.tmp.name)/'evidence';self.root.mkdir()
        self.record={'schema':1,'package':'ratatui','source_sha':h.SOURCE,'tree_sha':h.TREE,'target':h.TARGET,'rustc_commit':h.RUST,'rustc_sha256':'a'*64,'cargo_sha256':'b'*64,'target_sysroot_sha256':'c'*64,'binary_relative_path':'target/'+h.TARGET+'/release/pong-ratatui.exe','test_binary_relative_path':'target/'+h.TARGET+'/release/deps/test.exe'}
        self.record['evidence']={}
        for name in h.LOGS:
            content=b'test result: ok. 15 passed;\n' if name=='tests.log' else b'fixture'
            self.write(name,content); self.record['evidence'][name]=h.digest(self.root/name)
        for key,name in (('binary_sha256',self.record['binary_relative_path']),('test_binary_sha256',self.record['test_binary_relative_path'])):
            self.write(name,b'binary');self.record[key]=h.digest(self.root/name)
        self.save()
    def write(self,name,data):
        p=self.root/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
    def save(self): (self.root/'build-evidence.json').write_text(json.dumps(self.record),encoding='utf8')
    def test_verified_schema1_is_held_even_if_caller_invents_binding(self):
        self.assertEqual(h.verify_evidence(self.root)['package'],'ratatui')
        self.record['notice_binding']={'authenticated':True};self.save()
        output=self.root.parent/'release.zip'; called=[]
        with patch.object(h,'check_source'):
            with self.assertRaisesRegex(h.HandoffError,'lacks build-bound authenticated notice'):
                h.attempt_package(self.root.parent,self.root,output,lambda *args:called.append(args))
        self.assertFalse(called);self.assertFalse(output.exists())
    def test_modified_binary_or_log_refused(self):
        for name in (self.record['binary_relative_path'],'replay.jsonl'):
            original=(self.root/name).read_bytes();self.write(name,b'tampered')
            with self.assertRaisesRegex(h.HandoffError,'hash mismatch'):h.verify_evidence(self.root)
            self.write(name,original)
    def test_wrong_source_or_unsafe_test_path_refused(self):
        for key,value in (('source_sha','f'*40),('test_binary_relative_path','target/'+h.TARGET+'/release/deps/../../escape.exe')):
            original=self.record[key];self.record[key]=value;self.save()
            with self.assertRaises(h.HandoffError):h.verify_evidence(self.root)
            self.record[key]=original;self.save()
    def test_source_gate_rejects_dirty_and_wrong_tree(self):
        def runner(dirty=False,wrong=False):
            def run(cmd,**kw):
                if cmd[-1]=='HEAD': return h.SOURCE
                if cmd[-1]=='HEAD^{tree}': return 'f'*40 if wrong else h.TREE
                return '?? untracked' if dirty else ''
            return run
        h.check_source(self.root,runner())
        for run in (runner(dirty=True),runner(wrong=True)):
            with self.assertRaises(h.HandoffError):h.check_source(self.root,run)

    def test_bridge_uses_fixed_ratatui_source_and_restores_toolchain(self):
        calls=[]
        def producer(*args): calls.append(('producer',args,os.environ.get('RUSTUP_TOOLCHAIN')))
        def consumer(repo,evidence,output,packer):
            calls.append(('consumer',(repo,evidence,output),os.environ.get('RUSTUP_TOOLCHAIN')))
            raise h.HandoffError('unbound notices')
        with patch.dict(os.environ,{'RUSTUP_TOOLCHAIN':'previous'}):
            with self.assertRaisesRegex(h.HandoffError,'unbound notices'): bridge.run('repo','evidence','output',producer,consumer)
            self.assertEqual(os.environ['RUSTUP_TOOLCHAIN'],'previous')
        self.assertEqual(calls[0][1],('repo','evidence','ratatui',h.TARGET,h.SOURCE,h.TREE))
        self.assertEqual([c[2] for c in calls],['stable-x86_64-pc-windows-msvc']*2)
        self.assertEqual(calls[1][1],('repo','evidence','output'))

if __name__=='__main__':unittest.main()
