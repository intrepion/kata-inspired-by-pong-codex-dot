import copy
import hashlib
import io
import tempfile
import unittest
import zipfile
from pathlib import Path
from package_browser import FILES, package
from draft_browser_upload import ARCHIVE, SUMS, Failure, create_draft

class FakeTransport:
    def __init__(self): self.releases=[]; self.writes=[]; self.exists=False; self.publish_on_get=None; self.get_count=0; self.fail_after_upload=False
    def list_releases(self,page): return copy.deepcopy(self.releases) if page == 1 else []
    def tag_exists(self,tag): return self.exists
    def create(self,tag,sha):
        self.writes.append('create'); r={'id':17,'tag_name':tag,'target_commitish':sha,'draft':True,'prerelease':True,'assets':[]}; self.releases.append(r); return copy.deepcopy(r)
    def get(self,rid):
        self.get_count+=1
        if self.publish_on_get == self.get_count: self.releases[0]['draft']=False
        return copy.deepcopy(self.releases[0])
    def upload(self,rid,name,data):
        self.writes.append('upload:'+name)
        self.releases[0]['assets'].append({'name':name,'size':len(data),'digest':'sha256:'+hashlib.sha256(data).hexdigest(),'state':'uploaded'})
        if self.fail_after_upload: raise Failure('uncertain transport failure')

class DraftTests(unittest.TestCase):
    def setUp(self):
        self.tmp=tempfile.TemporaryDirectory();self.root=Path(self.tmp.name);self.src=self.root/'src';self.out=self.root/'out';self.out.mkdir();self.sha='a'*40
        for name in FILES:
            p=self.src/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text('fixture '+name)
        package(self.src,self.out/ARCHIVE,self.sha);self.refresh()
        self.t=FakeTransport()
    def tearDown(self): self.tmp.cleanup()
    def refresh(self):
        self.digest=hashlib.sha256((self.out/ARCHIVE).read_bytes()).hexdigest()
        (self.out/SUMS).write_text(f'{self.digest}  {ARCHIVE}\n')
    def run_upload(self,sha=None,digest=None): return create_draft(self.t,sha or self.sha,'123','1',self.out,digest or self.digest)
    def test_success_creates_only_draft_and_two_expected_assets(self):
        self.assertEqual(self.run_upload(),17);self.assertTrue(self.t.releases[0]['draft']);self.assertTrue(self.t.releases[0]['prerelease'])
        self.assertEqual(self.t.writes,['create','upload:'+ARCHIVE,'upload:'+SUMS])
    def test_wrong_source_or_digest_before_writes(self):
        for kwargs in ({'sha':'b'*40},{'digest':'f'*64}):
            with self.subTest(kwargs=kwargs), self.assertRaises(Failure):self.run_upload(**kwargs)
        self.assertEqual(self.t.writes,[])
    def test_bad_paths_extra_files_and_external_checksum_fail(self):
        (self.out/'extra').write_text('x')
        with self.assertRaisesRegex(Failure,'unexpected artifact'):self.run_upload()
        (self.out/'extra').unlink();(self.out/SUMS).write_text('bad')
        with self.assertRaisesRegex(Failure,'checksum'):self.run_upload()
        self.assertEqual(self.t.writes,[])
    def test_archive_traversal_duplicate_and_internal_checksum_fail(self):
        for mode in ('traversal','duplicate','checksum'):
            package_path=self.out/ARCHIVE; original=package_path.read_bytes()
            with zipfile.ZipFile(io.BytesIO(original)) as z: entries=[(i,z.read(i)) for i in z.infolist()]
            with zipfile.ZipFile(package_path,'w') as z:
                for i,data in entries:
                    if mode=='checksum' and i.filename.endswith('game.js'): data=b'changed'
                    z.writestr(i,data)
                if mode=='traversal':z.writestr('../outside',b'x')
                if mode=='duplicate':z.writestr(entries[0][0],entries[0][1])
            self.refresh()
            with self.subTest(mode=mode),self.assertRaises(Failure):self.run_upload()
            package_path.write_bytes(original);self.refresh()
        self.assertEqual(self.t.writes,[])
    def test_existing_draft_or_public_release_or_tag_fails(self):
        tag='draft-pong-'+self.sha+'-123-1'
        for draft in (True,False):
            self.t.releases=[{'tag_name':tag,'draft':draft}]
            with self.assertRaisesRegex(Failure,'already exists'):self.run_upload()
        self.t.releases=[];self.t.exists=True
        with self.assertRaisesRegex(Failure,'tag already exists'):self.run_upload()
        self.assertEqual(self.t.writes,[])
    def test_published_during_run_stops_before_first_upload(self):
        self.t.publish_on_get=1
        with self.assertRaisesRegex(Failure,'state or target'):self.run_upload()
        self.assertEqual(self.t.writes,['create'])
    def test_unexpected_existing_asset_or_wrong_target_is_rejected(self):
        from draft_browser_upload import ensure_release
        for change in ({'target_commitish':'b'*40},{'assets':[{'name':'foreign','size':1,'digest':'sha256:x','state':'uploaded'}]}):
            r={'id':17,'tag_name':'tag','target_commitish':self.sha,'draft':True,'prerelease':True,'assets':[]};r.update(change)
            with self.assertRaises(Failure):ensure_release(r,17,'tag',self.sha,{})
    def test_duplicate_and_wrong_digest_assets_fail(self):
        from draft_browser_upload import ensure_release
        r={'id':17,'tag_name':'tag','target_commitish':self.sha,'draft':True,'prerelease':True,
           'assets':[{'name':ARCHIVE,'size':1,'digest':'sha256:wrong','state':'uploaded'}]}
        with self.assertRaisesRegex(Failure,'digest'):ensure_release(r,17,'tag',self.sha,{ARCHIVE:(1,'sha256:expected')})
        r['assets'].append(copy.deepcopy(r['assets'][0]))
        with self.assertRaisesRegex(Failure,'duplicate'):ensure_release(r,17,'tag',self.sha,{})
    def test_public_transition_after_first_upload_stops_before_next(self):
        self.t.publish_on_get=3
        with self.assertRaisesRegex(Failure,'state or target'):self.run_upload()
        self.assertEqual(self.t.writes,['create','upload:'+ARCHIVE])
    def test_uncertain_partial_upload_is_not_retried_or_resumed(self):
        self.t.fail_after_upload=True
        with self.assertRaisesRegex(Failure,'uncertain'):self.run_upload()
        before=list(self.t.writes)
        with self.assertRaisesRegex(Failure,'already exists'):self.run_upload()
        self.assertEqual(self.t.writes,before)
    def test_main_rejects_untrusted_run_identity_and_trigger(self):
        import draft_browser_upload as uploader
        from unittest.mock import patch
        argv=['draft_browser_upload.py','--artifacts',str(self.out),'--sha',self.sha,'--run-id','123','--attempt','1','--expected-digest',self.digest]
        env={'GITHUB_REPOSITORY':uploader.REPO,'GITHUB_SHA':self.sha,'GITHUB_RUN_ID':'123','GITHUB_RUN_ATTEMPT':'1','GITHUB_REF':'refs/heads/main','GITHUB_EVENT_NAME':'workflow_dispatch','GH_TOKEN':'unused'}
        for key,value in [('GITHUB_SHA','b'*40),('GITHUB_RUN_ID','124'),('GITHUB_RUN_ATTEMPT','2'),('GITHUB_REF','refs/pull/1/merge'),('GITHUB_EVENT_NAME','pull_request'),('GITHUB_REPOSITORY','attacker/repo')]:
            changed=dict(env);changed[key]=value
            with self.subTest(key=key), patch.dict('os.environ',changed,clear=True), patch('sys.argv',argv), patch.object(uploader,'Transport') as transport:
                with self.assertRaises(SystemExit):uploader.main()
                transport.assert_not_called()
    def test_read_failure_before_creation_has_no_write(self):
        self.t.list_releases=lambda page: (_ for _ in ()).throw(Failure('transport'))
        with self.assertRaises(Failure):self.run_upload()
        self.assertEqual(self.t.writes,[])

if __name__=='__main__':unittest.main()
