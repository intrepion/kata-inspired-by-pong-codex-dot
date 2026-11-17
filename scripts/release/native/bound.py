"""Bound Ratatui Windows evidence candidate; local review, not release clearance."""
import argparse, hashlib, importlib.util, json, os, subprocess, sys
from pathlib import Path
import handoff
sys.path.insert(0,str(Path(__file__).resolve().parent/'producer'))
import produce
from produce import collect, EvidenceError

REVIEWED_LOCK='3cce5b2b1629f0bd517b4a85b368ed3632084af6e0e5f9f47ad49fbaa5456210'
REVIEWED_INDEX='a6d4a707228b3feea39616caec38a03367f4a555d26158f3fe68bd47f966dd19'
REVIEWED_TEXT='115470a57bd719dcdccfd53815c8c43180ea3b459f2094c7c015c8f9257aad0d'
RUSTUP='stable-x86_64-pc-windows-msvc'

def sha(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def require_equal(actual,expected,label):
    if actual!=expected: raise handoff.HandoffError(label+' mismatch')

def supplements():
    root=Path(__file__).resolve().parent.parent/'windows'
    spec=importlib.util.spec_from_file_location('windows_release_package',root/'package.py')
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module)
    manifest_path=root/'inputs-manifest.json';manifest_bytes=manifest_path.read_bytes()
    require_equal(hashlib.sha256(manifest_bytes).hexdigest(),module.MANIFEST_SHA256,'supplement manifest')
    manifest=json.loads(manifest_bytes)
    if manifest['target']!=handoff.TARGET or manifest['rust_commit']!=handoff.RUST: raise handoff.HandoffError('supplement compiler or target mismatch')
    files=module.verified_inputs(manifest,root/'inputs','ratatui')
    digest=hashlib.sha256(''.join(f'{name} {hashlib.sha256(data).hexdigest()}\n' for name,data in sorted(files.items())).encode()).hexdigest()
    return {'manifest_sha256':module.MANIFEST_SHA256,'files_sha256':digest,'count':len(files)}

def capture(repo,work,phase,env,cargo):
    project=repo/'experiments/ratatui'; meta=work/(phase+'-metadata.json'); prefix=work/(phase+'-notices')
    args=[str(cargo),'metadata','--locked','--offline','--filter-platform',handoff.TARGET,'--format-version','1','--manifest-path',str(project/'Cargo.toml')]
    result=subprocess.run(args,cwd=repo,env=env,check=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=300)
    meta.write_bytes(result.stdout); (work/(phase+'-metadata.stderr')).write_bytes(result.stderr)
    driver=Path(__file__).resolve().parent/'generate-notice-probe.mjs'
    node=subprocess.run(['node',str(driver),str(repo),str(meta),str(prefix)],cwd=repo,env=env,check=True,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=300)
    (work/(phase+'-notice-collector.log')).write_bytes(node.stdout+node.stderr)
    idx=Path(str(prefix)+'.json');text=Path(str(prefix)+'.txt')
    require_equal(sha(idx),REVIEWED_INDEX,phase+' notice index');require_equal(sha(text),REVIEWED_TEXT,phase+' notice text')
    data=json.loads(meta.read_text(encoding='utf8')); index=json.loads(idx.read_text(encoding='utf8'))
    packages={(p['name'],p['version'],p['license']) for p in data['packages']}
    listed={(p['name'],p['version'],p['license']) for p in index['packages']}
    if not packages or packages!=listed or index.get('missing_notice_files'): raise handoff.HandoffError('incomplete target notice graph')
    return {'metadata_sha256':sha(meta),'index_sha256':sha(idx),'text_sha256':sha(text),'package_count':len(packages)}

def run(repo,work):
    repo=Path(repo).resolve(strict=True); work=Path(work).resolve()
    if work.exists() or not work.parent.is_dir() or repo==work.resolve() or repo in work.resolve().parents: raise handoff.HandoffError('new output outside source required')
    handoff.check_source(repo); lock=repo/'experiments/ratatui/Cargo.lock'; require_equal(sha(lock),REVIEWED_LOCK,'reviewed lockfile')
    env=os.environ.copy();env['RUSTUP_TOOLCHAIN']=RUSTUP
    for key in list(env):
        if key in {'RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUSTC','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER'} or key.startswith(('CARGO_BUILD_','CARGO_TARGET_')): env.pop(key,None)
    env['RUSTC_WRAPPER']='';env['RUSTC_WORKSPACE_WRAPPER']='';env['CARGO_NET_OFFLINE']='true'
    sysroot=Path(subprocess.check_output(['rustc','--print','sysroot'],cwd=repo,env=env,text=True).strip()).resolve(strict=True)
    rustc=sysroot/'bin/rustc.exe';cargo=sysroot/'bin/cargo.exe'
    version=subprocess.check_output([str(rustc),'-vV'],cwd=repo,env=env,text=True)
    if ('commit-hash: '+handoff.RUST) not in version or ('host: '+handoff.TARGET) not in version: raise handoff.HandoffError('compiler identity mismatch')
    compiler_hash=sha(rustc);cargo_hash=sha(cargo)
    work.mkdir(exist_ok=False); build=work/'build';env['RUSTC']=str(rustc);env['CARGO_TARGET_DIR']=str(build/'target');env['CARGO_ENCODED_RUSTFLAGS']='--remap-path-prefix='+str(Path.home())+'=/user\x1f--remap-path-prefix='+str(repo)+'=/src'
    binding_path=Path(__file__).resolve(strict=True);binding_hash=sha(binding_path)
    producer_path=Path(produce.__file__).resolve(strict=True);producer_hash=sha(producer_path)
    handoff_path=Path(handoff.__file__).resolve(strict=True);handoff_hash=sha(handoff_path)
    collector=repo/'scripts/packaging/notice-bundle.mjs';collector_hash=sha(collector)
    driver=Path(__file__).resolve().parent/'generate-notice-probe.mjs';driver_hash=sha(driver)
    supplement_before=supplements()
    before=capture(repo,work,'before',env,cargo)
    previous=os.environ.get('RUSTUP_TOOLCHAIN');os.environ['RUSTUP_TOOLCHAIN']=RUSTUP
    try: record=collect(repo,build,'ratatui',handoff.TARGET,handoff.SOURCE,handoff.TREE)
    finally:
        if previous is None: os.environ.pop('RUSTUP_TOOLCHAIN',None)
        else: os.environ['RUSTUP_TOOLCHAIN']=previous
    require_equal(record['rustc_sha256'],compiler_hash,'producer compiler');require_equal(record['cargo_sha256'],cargo_hash,'producer Cargo')
    after=capture(repo,work,'after',env,cargo);require_equal(before,after,'before/after notice binding')
    require_equal(sha(binding_path),binding_hash,'post-build binding code')
    require_equal(sha(producer_path),producer_hash,'post-build producer code')
    require_equal(sha(handoff_path),handoff_hash,'post-build handoff code')
    require_equal(supplements(),supplement_before,'post-build supplements')
    require_equal(sha(lock),REVIEWED_LOCK,'post-build lockfile');require_equal(sha(collector),collector_hash,'collector');require_equal(sha(driver),driver_hash,'notice driver')
    require_equal(sha(rustc),compiler_hash,'post-build compiler');require_equal(sha(cargo),cargo_hash,'post-build Cargo');handoff.check_source(repo)
    handoff.verify_evidence(build)
    binding={'schema':2,'package':'ratatui','source_sha':handoff.SOURCE,'tree_sha':handoff.TREE,'target':handoff.TARGET,'lockfile_sha256':REVIEWED_LOCK,'reviewed_index_sha256':REVIEWED_INDEX,'reviewed_text_sha256':REVIEWED_TEXT,'metadata_sha256':before['metadata_sha256'],'notice_index_sha256':before['index_sha256'],'notice_text_sha256':before['text_sha256'],'package_count':before['package_count'],'supplements':supplement_before,'binding_code_sha256':binding_hash,'producer_code_sha256':producer_hash,'handoff_code_sha256':handoff_hash,'collector_sha256':collector_hash,'notice_driver_sha256':driver_hash,'producer_record_sha256':sha(build/'build-evidence.json'),'binary_sha256':record['binary_sha256'],'rustc_sha256':compiler_hash,'cargo_sha256':cargo_hash,'limitations':'Local process evidence, not attestation. Target metadata includes build dependencies and is not proof of final linkage. Prior reviewed notice bytes and source-specific uncertainties remain; no legal clearance, privacy, signing, clean-machine or physical-input qualification.'}
    (work/'bound-evidence.json').write_text(json.dumps(binding,indent=2,sort_keys=True)+'\n',encoding='utf8')
    return binding

def main():
    p=argparse.ArgumentParser();p.add_argument('--repo',required=True);p.add_argument('--output',required=True);a=p.parse_args();b=run(a.repo,a.output);print(json.dumps({'schema':b['schema'],'binary_sha256':b['binary_sha256'],'notice_review':'bounded prior-reviewed bytes; no clearance'}))
if __name__=='__main__':
    try: main()
    except (handoff.HandoffError,EvidenceError,OSError,ValueError,KeyError,subprocess.CalledProcessError,subprocess.TimeoutExpired) as e: print('bound producer refused:',e,file=sys.stderr);sys.exit(1)
