import importlib.util, json, struct, tempfile, unittest, zipfile
from pathlib import Path
spec=importlib.util.spec_from_file_location('windows_package',Path(__file__).with_name('package.py'))
p=importlib.util.module_from_spec(spec); spec.loader.exec_module(p)

class PackagingTests(unittest.TestCase):
    def test_exact_toolchain_and_reviewed_manifest(self):
        source = Path(__file__).with_name('inputs-manifest.json')
        data = source.read_bytes()
        reviewed = p.reviewed_manifest(data)
        self.assertEqual(p.TOOLCHAIN, reviewed['rust_release'] + '-' + reviewed['host'])
        self.assertEqual(p.TOOLCHAIN, '1.100.0-x86_64-pc-windows-msvc')
        stale = data.replace(b'1.100.0', b'1.99.1', 1)
        with self.assertRaisesRegex(ValueError, 'manifest changed'):
            p.reviewed_manifest(stale)
        old = 'release: 1.99.1\ncommit-hash: 58cf8f9c35b355199727627d07ae34a4e2846d67\nhost: x86_64-pc-windows-msvc\n'
        with self.assertRaisesRegex(ValueError, 'compiler identity'):
            p.verify_compiler(old, reviewed)
        notices=p.verified_inputs(reviewed, source.parent/'inputs', 'sdl3')
        self.assertIn('notices/rust/COPYRIGHT-library.html', notices)
        self.assertNotIn('notices/unicode/EastAsianWidth-15.1.0-header.txt', notices)

    def test_pe_machine_and_format(self):
        b=bytearray(154); b[:2]=b'MZ'; struct.pack_into('<I',b,0x3c,128); b[128:132]=b'PE\0\0'; struct.pack_into('<HHIIIHH',b,132,0x8664,0,0,0,0,2,2); struct.pack_into('<H',b,152,0x20b)
        p.verify_pe_x64(b)
        for bad in (b'not PE',b[:130],b[:132]+struct.pack('<H',0x14c)+b[134:]):
            with self.assertRaises(ValueError): p.verify_pe_x64(bad)

    def test_compiler_identity_is_strict(self):
        manifest={'rust_release':'1.99.1','rust_commit':'abc','host':'x86_64-pc-windows-msvc'}
        good='release: 1.99.1\ncommit-hash: abc\nhost: x86_64-pc-windows-msvc\n'
        p.verify_compiler(good,manifest)
        for bad in (good.replace('abc','other'),good.replace('1.99.1','1.99.2'),good.replace('msvc','gnu')):
            with self.assertRaises(ValueError): p.verify_compiler(bad,manifest)

    def test_cargo_and_compiler_share_project_context_and_selection(self):
        manifest={'rust_release':'1.99.1','rust_commit':'abc','host':'x86_64-pc-windows-msvc'}
        identity='release: 1.99.1\ncommit-hash: abc\nhost: x86_64-pc-windows-msvc\n'
        with tempfile.TemporaryDirectory() as d:
            project=Path(d)/'project'; project.mkdir(); compiler=Path(d)/'rustc.exe'; compiler.write_bytes(b'review compiler')
            calls=[]
            def run(cmd,**kw):
                calls.append((cmd,kw))
                if cmd[0]=='rustup': return str(compiler)+'\n'
                if cmd[0]==str(compiler): return identity
                if cmd[0]=='cargo': return '{"packages":[]}'
                raise AssertionError(cmd)
            env,evidence=p.compiler_context(project,manifest,run)
            p.current_metadata(project,env,run)
            self.assertEqual([call[1]['cwd'] for call in calls],[project,project,project])
            self.assertEqual(calls[0][0],['rustup','which','--toolchain',p.TOOLCHAIN,'rustc'])
            self.assertEqual(calls[1][0],[str(compiler),'-Vv'])
            self.assertEqual(calls[2][0][1],'+'+p.TOOLCHAIN)
            self.assertEqual(calls[2][1]['env']['RUSTC'],str(compiler))
            self.assertEqual(calls[2][1]['env']['RUSTC_WRAPPER'],'')
            self.assertEqual(evidence['rustc_sha256'],p.sha(b'review compiler'))
            def mismatch(cmd,**kw): return str(compiler) if cmd[0]=='rustup' else identity.replace('commit-hash: abc','commit-hash: wrong')
            with self.assertRaises(ValueError): p.compiler_context(project,manifest,mismatch)

    def test_inventory_rejects_missing_mismatch_and_excluded_license(self):
        text=b'license terms\n'; pkg={'name':'crate','version':'1','license':'MIT'}
        meta={'packages':[pkg]}; idx={'packages':[dict(pkg,notices=[{'source':'LICENSE'}])],'bundle_sha256':p.sha(text),'missing_notice_files':[]}
        p.verify_inventory(meta,idx,text)
        for broken in (dict(idx,missing_notice_files=['crate@1']),dict(idx,bundle_sha256='0'*64),dict(idx,packages=[dict(pkg,notices=[])]),dict(idx,packages=[dict(pkg,version='2',notices=[1])])):
            with self.assertRaises(ValueError): p.verify_inventory(meta,broken,text)
        excluded=dict(pkg,license='MPL-2.0'); x={'packages':[excluded]}; y={'packages':[dict(excluded,notices=[1])],'bundle_sha256':p.sha(text),'missing_notice_files':[]}
        with self.assertRaises(ValueError): p.verify_inventory(x,y,text)

    def test_supplement_tampering_or_absence_fails(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d); (root/'a').write_bytes(b'terms'); manifest={'files':{'a':p.sha(b'terms')}}
            self.assertEqual(p.verified_inputs(manifest,root,'softbuffer')['notices/a'],b'terms')
            (root/'a').write_bytes(b'changed')
            with self.assertRaises(ValueError): p.verified_inputs(manifest,root,'softbuffer')
            (root/'a').unlink()
            with self.assertRaises(FileNotFoundError): p.verified_inputs(manifest,root,'softbuffer')

    def test_zip_paths_privacy_checksums_and_no_overwrite(self):
        with tempfile.TemporaryDirectory() as d:
            output=Path(d)/'archive.zip'; contents={'pong.exe':b'fresh binary','notices/a.txt':b'license'}
            digest=p.make_zip(output,contents,['private-user']); self.assertEqual(digest,p.sha(output.read_bytes()))
            with zipfile.ZipFile(output) as z:
                self.assertIsNone(z.testzip()); checks=z.read('SHA256SUMS').decode()
                for name,data in contents.items(): self.assertIn(p.sha(data)+'  '+name,checks)
                self.assertTrue(all(i.date_time==(2020,1,1,0,0,0) for i in z.infolist()))
            with self.assertRaises(FileExistsError): p.make_zip(output,contents,['private-user'])
            for name,data in (('../escape',b'ok'),('safe',b'C:/PRIVATE-USER/data'),('safe','PRIVATE-USER'.encode('utf-16le'))):
                with self.assertRaises(ValueError): p.make_zip(Path(d)/'bad.zip',{name:data},['private-user'])

if __name__=='__main__': unittest.main()
