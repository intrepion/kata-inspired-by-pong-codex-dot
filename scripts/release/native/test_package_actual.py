import json, os, shutil, subprocess, tempfile, unittest
from pathlib import Path
import handoff, package_bound, verify_bound

# Optional external inputs must describe a verified, clean source and evidence pair.
# No local machine paths or binaries are part of the fixture.
REPO=os.environ.get('PONG_RATATUI_TEST_REPO')
EVIDENCE=os.environ.get('PONG_RATATUI_TEST_EVIDENCE')

@unittest.skipUnless(REPO and EVIDENCE, 'set PONG_RATATUI_TEST_REPO and PONG_RATATUI_TEST_EVIDENCE for live-evidence tests')
class ActualHandoffFailures(unittest.TestCase):
 def setUp(self):
  self.temp=tempfile.TemporaryDirectory();self.addCleanup(self.temp.cleanup)
  self.root=Path(self.temp.name);self.work=self.root/'evidence';self.work.mkdir()
  record=json.loads((Path(EVIDENCE)/'build/build-evidence.json').read_text())
  names=['bound-evidence.json','before-metadata.json','after-metadata.json','before-notices.json','after-notices.json','before-notices.txt','after-notices.txt','build/build-evidence.json']
  names += ['build/'+n for n in record['evidence']]+['build/'+record['binary_relative_path'],'build/'+record['test_binary_relative_path']]
  for name in names:
   dst=self.work/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(Path(EVIDENCE)/name,dst)
  verify_bound.verify(REPO, self.work)
  self.output=self.root/'candidate.zip'
 def refused(self, reason):
  with self.assertRaisesRegex(handoff.HandoffError, reason):package_bound.package(REPO,self.work,self.output,['verified-checkout'])
  self.assertFalse(self.output.exists())
 def test_old_schema(self):
  p=self.work/'bound-evidence.json';r=json.loads(p.read_text());r['schema']=1;p.write_text(json.dumps(r));self.refused('bound source, target or schema mismatch')
 def test_notice_tampering(self):
  (self.work/'before-notices.txt').write_bytes(b'tampered');self.refused('before notice text mismatch')
 def test_binary_tampering(self):
  r=json.loads((self.work/'build/build-evidence.json').read_text());(self.work/'build'/r['binary_relative_path']).write_bytes(b'tampered');self.refused('evidence hash mismatch')
 def test_log_tampering(self):
  (self.work/'build/tests.log').write_bytes(b'tampered');self.refused('evidence hash mismatch')
 def test_graph_tampering(self):
  (self.work/'before-metadata.json').write_bytes(b'{}');self.refused('before metadata mismatch')
 def test_wrong_source(self):
  wrong=self.root/'wrong-source';wrong.mkdir()
  with self.assertRaises(subprocess.CalledProcessError):package_bound.package(wrong,self.work,self.output,['verified-checkout'])
  self.assertFalse(self.output.exists())

if __name__=='__main__':unittest.main()
