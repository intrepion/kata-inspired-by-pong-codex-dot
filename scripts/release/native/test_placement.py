import tempfile, unittest
from pathlib import Path
import package_bound, handoff
class PlacementTests(unittest.TestCase):
 def setUp(self):
  self.t=tempfile.TemporaryDirectory(dir=Path(__file__).parent);self.addCleanup(self.t.cleanup)
  self.root=Path(self.t.name);self.src=self.root/'in.zip';self.dst=self.root/'out.zip';self.src.write_bytes(b'archive bytes')
 def test_success(self):
  self.assertEqual(package_bound.place_verified(self.src,self.dst),package_bound.bound.sha(self.src));self.assertEqual(self.src.read_bytes(),self.dst.read_bytes())
 def test_collision_preserved(self):
  self.dst.write_bytes(b'keep')
  with self.assertRaises(FileExistsError):package_bound.place_verified(self.src,self.dst)
  self.assertEqual(self.dst.read_bytes(),b'keep')
 def test_interrupted_copy_cleanup(self):
  def partial(src,dst):dst.write(b'partial');raise OSError('copy interrupted')
  with self.assertRaises(OSError):package_bound.place_verified(self.src,self.dst,partial)
  self.assertFalse(self.dst.exists())
 def test_short_copy_cleanup(self):
  def partial(src,dst):dst.write(b'partial')
  with self.assertRaises(handoff.HandoffError):package_bound.place_verified(self.src,self.dst,partial)
  self.assertFalse(self.dst.exists())
if __name__=='__main__':unittest.main()
