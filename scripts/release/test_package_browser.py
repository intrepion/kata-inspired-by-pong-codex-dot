import hashlib
import tempfile
import unittest
import zipfile
from unittest.mock import patch
from pathlib import Path
from package_browser import FILES, ROOT, package

class PackageTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.root = Path(self.tmp.name)
        self.src = self.root / 'src'
        for name in FILES:
            p = self.src / name
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text('fixture ' + name)
        self.out = self.root / 'out' / 'browser.zip'
        self.sha = 'a' * 40
    def tearDown(self): self.tmp.cleanup()
    def test_archive_and_manifest(self):
        package(self.src, self.out, self.sha)
        with zipfile.ZipFile(self.out) as z:
            self.assertIsNone(z.testzip())
            self.assertEqual(set(z.namelist()), {ROOT + x for x in (*FILES, 'README.txt', 'SHA256SUMS')})
            for line in z.read(ROOT + 'SHA256SUMS').decode().splitlines():
                digest, name = line.split('  ./', 1)
                self.assertEqual(digest, hashlib.sha256(z.read(ROOT + name)).hexdigest())
            self.assertIn(self.sha.encode(), z.read(ROOT+'README.txt'))
    def test_rejects_bad_sha_and_existing_output(self):
        with self.assertRaises(ValueError): package(self.src, self.out, 'main')
        package(self.src, self.out, self.sha)
        with self.assertRaises(ValueError): package(self.src, self.out, self.sha)
    def test_runner_paths_and_encodings_are_rejected(self):
        values={'GITHUB_WORKSPACE':'/home/build/private-workspace/repository',
                'HOME':'/home/build/private-home',
                'USERPROFILE':'C:\\Users\\PrivatePerson',
                'RUNNER_TEMP':'/tmp/private-runner-output'}
        for label,value in values.items():
            for encoding in ('utf-8','utf-16le','utf-16be'):
                with self.subTest(label=label,encoding=encoding), patch.dict('os.environ', values):
                    (self.src/'game.js').write_bytes(value.encode(encoding))
                    with self.assertRaisesRegex(ValueError,'private build path'):
                        package(self.src,self.out,self.sha)
        for path in (self.src.resolve(), self.out.resolve(), self.out.parent.resolve()):
            for encoding in ('utf-8', 'utf-16le', 'utf-16be'):
                with self.subTest(actual_path=path.name,encoding=encoding):
                    (self.src/'game.js').write_bytes(str(path).encode(encoding))
                    with self.assertRaisesRegex(ValueError,'private build path'):
                        package(self.src,self.out,self.sha)
        with patch.dict('os.environ', values):
            (self.src/'game.js').write_text('private-workspace/repository')
            with self.assertRaisesRegex(ValueError,'private build path'):
                package(self.src,self.out,self.sha)
            (self.src/'game.js').write_bytes(str(self.out).encode('utf-16le'))
            with self.assertRaisesRegex(ValueError,'private build path'):
                package(self.src,self.out,self.sha)
    def test_rejects_symlink_and_private_path(self):
        p = self.src / 'game.js'; p.unlink(); p.symlink_to(self.src / 'LICENSE')
        with self.assertRaises(ValueError): package(self.src, self.out, self.sha)
        p.unlink(); p.write_text('/home/runner/work/private')
        with self.assertRaises(ValueError): package(self.src, self.out, self.sha)

if __name__ == '__main__': unittest.main()
