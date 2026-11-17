"""Validate local schema-2 binding; no publication or clearance decision."""
import json
from pathlib import Path
import bound, handoff

def verify(repo,work):
    repo=Path(repo).resolve(strict=True);work=Path(work).resolve(strict=True)
    handoff.check_source(repo)
    record=json.loads(handoff.safe_file(work,'bound-evidence.json').read_text(encoding='utf8'))
    if (record.get('schema'),record.get('package'),record.get('source_sha'),record.get('tree_sha'),record.get('target'))!=(2,'ratatui',handoff.SOURCE,handoff.TREE,handoff.TARGET):
        raise handoff.HandoffError('bound source, target or schema mismatch')
    def eq(a,b,label): bound.require_equal(a,b,label)
    eq(bound.sha(repo/'experiments/ratatui/Cargo.lock'),bound.REVIEWED_LOCK,'source lockfile')
    eq(record.get('lockfile_sha256'),bound.REVIEWED_LOCK,'recorded lockfile')
    eq(record.get('reviewed_index_sha256'),bound.REVIEWED_INDEX,'reviewed index')
    eq(record.get('reviewed_text_sha256'),bound.REVIEWED_TEXT,'reviewed notice text')
    eq(record.get('notice_index_sha256'),bound.REVIEWED_INDEX,'notice index')
    eq(record.get('notice_text_sha256'),bound.REVIEWED_TEXT,'notice text')
    eq(record.get('supplements'),bound.supplements(),'supplements')
    eq(record.get('binding_code_sha256'),bound.sha(bound.__file__),'binding code')
    eq(record.get('producer_code_sha256'),bound.sha(bound.produce.__file__),'producer code')
    eq(record.get('handoff_code_sha256'),bound.sha(handoff.__file__),'handoff code')
    for phase in ('before','after'):
        eq(bound.sha(handoff.safe_file(work,phase+'-metadata.json')),record.get('metadata_sha256'),phase+' metadata')
        eq(bound.sha(handoff.safe_file(work,phase+'-notices.json')),bound.REVIEWED_INDEX,phase+' notice index')
        eq(bound.sha(handoff.safe_file(work,phase+'-notices.txt')),bound.REVIEWED_TEXT,phase+' notice text')
    eq(bound.sha(repo/'scripts/packaging/notice-bundle.mjs'),record.get('collector_sha256'),'source collector')
    eq(bound.sha(Path(__file__).resolve().parent/'generate-notice-probe.mjs'),record.get('notice_driver_sha256'),'notice driver')
    build=work/'build'; original=handoff.verify_evidence(build)
    eq(bound.sha(handoff.safe_file(build,'build-evidence.json')),record.get('producer_record_sha256'),'producer record')
    for key in ('binary_sha256','rustc_sha256','cargo_sha256'): eq(original.get(key),record.get(key),key)
    return record
