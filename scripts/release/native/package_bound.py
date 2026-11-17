"""Local schema-2 Ratatui package. A review candidate, not release clearance."""
import argparse, importlib.util, json, os, shutil, sys, tempfile
from pathlib import Path
import bound, handoff, verify_bound

def place_verified(source, output, copy=shutil.copyfileobj):
    source=Path(source);output=Path(output);expected=bound.sha(source);created=False
    try:
        with source.open('rb') as src, output.open('xb') as dst:
            created=True
            copy(src,dst)
        if bound.sha(output)!=expected: raise handoff.HandoffError('placed archive hash mismatch')
        return expected
    except BaseException:
        if created: output.unlink(missing_ok=True)
        raise

def package(repo,work,output,forbidden):
    repo=Path(repo).resolve(strict=True);work=Path(work).resolve(strict=True);output=Path(output).resolve()
    if output.exists() or repo==output or repo in output.parents or work==output or work in output.parents: raise handoff.HandoffError('output must be new and outside source and evidence')
    if not forbidden or any(len(s)<4 for s in forbidden): raise handoff.HandoffError('explicit privacy markers required')
    record=verify_bound.verify(repo,work)
    build=work/'build'; original=handoff.verify_evidence(build)
    binary=handoff.safe_file(build,original['binary_relative_path']).read_bytes()
    index=handoff.safe_file(work,'before-notices.json').read_bytes();text=handoff.safe_file(work,'before-notices.txt').read_bytes()
    metadata=json.loads(handoff.safe_file(work,'before-metadata.json').read_text(encoding='utf8'))
    root=Path(__file__).resolve().parent.parent/'windows'
    spec=importlib.util.spec_from_file_location('windows_release_package',root/'package.py')
    pack=importlib.util.module_from_spec(spec);spec.loader.exec_module(pack)
    pack.verify_pe_x64(binary);pack.verify_inventory(metadata,json.loads(index),text)
    manifest_bytes=(root/'inputs-manifest.json').read_bytes();manifest=json.loads(manifest_bytes)
    supplements=pack.verified_inputs(manifest,root/'inputs','ratatui')
    provenance={'source_commit':handoff.SOURCE,'source_tree':handoff.TREE,'target':handoff.TARGET,'evidence_schema':2,'binary_sha256':record['binary_sha256'],'lockfile_sha256':record['lockfile_sha256'],'notice_index_sha256':record['notice_index_sha256'],'notice_text_sha256':record['notice_text_sha256'],'supplements':record['supplements'],'rustc_sha256':record['rustc_sha256'],'cargo_sha256':record['cargo_sha256'],'note':'Local process evidence, not independent attestation; target graph includes build dependencies and is not proof of final linkage.'}
    data={'pong-ratatui.exe':binary,'LICENSE':(repo/'LICENSE').read_bytes(),'notices/dependencies.txt':text,'notices/dependencies-index.json':index,'notices/inputs-manifest.json':manifest_bytes,'PROVENANCE.json':(json.dumps(provenance,sort_keys=True,indent=2)+'\n').encode(),'RUNNING.txt':('Windows x64 experimental review candidate; unsigned. Compatible Microsoft Visual C++ v14 runtime and UCRT may be required. Requires a terminal at least 50 columns by 18 rows. Physical input and clean-machine behavior are unverified. The notice graph includes build dependencies and does not prove final linkage. Prior reviewed notices retain their stated provenance and limitations; no legal clearance is claimed.\n').encode()}
    data.update(supplements)
    markers=set(forbidden)
    for path in (repo,work,Path.home()):
        markers.add(str(path));markers.add(str(path).replace('\\','/'))
    output.parent.mkdir(parents=False,exist_ok=True)
    fd,tmp=tempfile.mkstemp(prefix='.ratatui-package-',suffix='.zip',dir=output.parent);os.close(fd);os.unlink(tmp)
    try:
        digest=pack.make_zip(tmp,data,markers)
        # Recheck all bound inputs after constructing the archive.
        verify_bound.verify(repo,work)
        if output.exists(): raise handoff.HandoffError('output appeared during packaging')
        placed=place_verified(tmp,output)
        if placed!=digest:
            output.unlink(missing_ok=True)
            raise handoff.HandoffError('placed archive differs from verified ZIP')
    finally:
        if os.path.exists(tmp): os.unlink(tmp)
    return digest

def main():
    p=argparse.ArgumentParser();p.add_argument('--repo',required=True);p.add_argument('--evidence',required=True);p.add_argument('--output',required=True);p.add_argument('--forbid',action='append',required=True);a=p.parse_args()
    print(package(a.repo,a.evidence,a.output,a.forbid))
if __name__=='__main__':
    try:main()
    except (ValueError,OSError,KeyError) as e:print('packaging refused:',e,file=sys.stderr);sys.exit(1)
