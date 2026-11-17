import json, tempfile, unittest
from pathlib import Path
from unittest.mock import patch
import bound, handoff, verify_bound

class BindingTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory(); self.addCleanup(self.tmp.cleanup)
        root=Path(self.tmp.name); self.repo=root/'repo';self.work=root/'evidence'
        (self.repo/'experiments/ratatui').mkdir(parents=True);(self.repo/'scripts/packaging').mkdir(parents=True);(self.work/'build').mkdir(parents=True)
        (self.repo/'experiments/ratatui/Cargo.lock').write_bytes(b'lock');(self.repo/'scripts/packaging/notice-bundle.mjs').write_bytes(b'collector')
        for phase in ('before','after'):
            for name,data in [('metadata.json',b'meta'),('notices.json',b'index'),('notices.txt',b'text')]: (self.work/(phase+'-'+name)).write_bytes(data)
        (self.work/'build/build-evidence.json').write_bytes(b'producer')
        self.original={'binary_sha256':'binary','rustc_sha256':'rustc','cargo_sha256':'cargo'}
        self.supp={'manifest_sha256':'m','files_sha256':'f','count':2}
        self.record=dict(schema=2,package='ratatui',source_sha=handoff.SOURCE,tree_sha=handoff.TREE,target=handoff.TARGET,lockfile_sha256=bound.sha(self.repo/'experiments/ratatui/Cargo.lock'),reviewed_index_sha256=bound.sha(self.work/'before-notices.json'),reviewed_text_sha256=bound.sha(self.work/'before-notices.txt'),metadata_sha256=bound.sha(self.work/'before-metadata.json'),notice_index_sha256=bound.sha(self.work/'before-notices.json'),notice_text_sha256=bound.sha(self.work/'before-notices.txt'),binding_code_sha256=bound.sha(bound.__file__),producer_code_sha256=bound.sha(bound.produce.__file__),handoff_code_sha256=bound.sha(handoff.__file__),collector_sha256=bound.sha(self.repo/'scripts/packaging/notice-bundle.mjs'),notice_driver_sha256=bound.sha(Path(bound.__file__).parent/'generate-notice-probe.mjs'),producer_record_sha256=bound.sha(self.work/'build/build-evidence.json'),supplements=self.supp,**self.original)
        for obj,name,value in [(bound,'REVIEWED_LOCK',self.record['lockfile_sha256']),(bound,'REVIEWED_INDEX',self.record['reviewed_index_sha256']),(bound,'REVIEWED_TEXT',self.record['reviewed_text_sha256']),(bound,'supplements',lambda:self.supp),(handoff,'check_source',lambda repo:None),(handoff,'verify_evidence',lambda work:self.original)]:
            x=patch.object(obj,name,value);x.start();self.addCleanup(x.stop)
    def write(self): (self.work/'bound-evidence.json').write_text(json.dumps(self.record))
    def test_valid(self): self.write();self.assertEqual(verify_bound.verify(self.repo,self.work)['schema'],2)
    def test_record_mismatches_fail_closed(self):
        for key,value in [('schema',1),('producer_code_sha256','other'),('handoff_code_sha256','other'),('binding_code_sha256','other'),('source_sha','other'),('tree_sha','other'),('target','other'),('lockfile_sha256','other'),('binary_sha256','other'),('metadata_sha256','other'),('notice_index_sha256','other'),('notice_text_sha256','other'),('supplements',{}),('producer_record_sha256','other')]:
            with self.subTest(key=key):
                original=self.record[key];self.record[key]=value;self.write()
                with self.assertRaises(handoff.HandoffError): verify_bound.verify(self.repo,self.work)
                self.record[key]=original
    def test_mutated_files_fail_closed(self):
        for path in [self.repo/'experiments/ratatui/Cargo.lock',self.work/'before-metadata.json',self.work/'after-metadata.json',self.work/'before-notices.json',self.work/'after-notices.txt',self.work/'build/build-evidence.json']:
            with self.subTest(path=path.name):
                self.write();original=path.read_bytes();path.write_bytes(b'tampered')
                with self.assertRaises(handoff.HandoffError): verify_bound.verify(self.repo,self.work)
                path.write_bytes(original)
    def test_missing_binding_fails(self):
        del self.record['supplements'];self.write()
        with self.assertRaises(handoff.HandoffError): verify_bound.verify(self.repo,self.work)
    def test_changed_producer_code_fails(self):
        self.write(); changed=Path(self.tmp.name)/'produce-changed.py';changed.write_text('changed')
        with patch.object(bound.produce,'__file__',str(changed)):
            with self.assertRaises(handoff.HandoffError):verify_bound.verify(self.repo,self.work)
    def test_source_check_failure_propagates(self):
        self.write()
        with patch.object(handoff,'check_source',side_effect=handoff.HandoffError('source dirty')):
            with self.assertRaises(handoff.HandoffError):verify_bound.verify(self.repo,self.work)

if __name__=='__main__': unittest.main()
