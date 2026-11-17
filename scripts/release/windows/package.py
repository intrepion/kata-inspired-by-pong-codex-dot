"""Create a review ZIP from fresh inputs; matching identity is not binary attestation."""
import argparse, hashlib, json, os, struct, subprocess, sys, zipfile
from pathlib import Path, PurePosixPath

HERE=Path(__file__).resolve().parent
COMMIT='637d5a16f3ccdb357b397639ffe1a147bb107cb0'
TREE='26b8283885ebfce73eade8d3be672cf5dcfbffea'
TARGET='x86_64-pc-windows-msvc'
TOOLCHAIN='stable-x86_64-pc-windows-msvc'
MANIFEST_SHA256='c3b3952f7f2b93160c750cc5f70e1defd238d85af702f198077c88b190f4fef5'

def sha(data): return hashlib.sha256(data).hexdigest()

def verify_pe_x64(data):
    if len(data)<64 or data[:2]!=b'MZ': raise ValueError('binary is not a PE executable')
    offset=struct.unpack_from('<I',data,0x3c)[0]
    if offset+24>len(data) or data[offset:offset+4]!=b'PE\0\0': raise ValueError('invalid PE header')
    machine,_,_,_,_,optional_size,flags=struct.unpack_from('<HHIIIHH',data,offset+4)
    if machine!=0x8664 or not flags&0x0002 or flags&0x2000 or optional_size<2 or offset+24+optional_size>len(data) or struct.unpack_from('<H',data,offset+24)[0]!=0x20b:
        raise ValueError('binary is not an x64 PE executable')

def verify_compiler(text, manifest):
    fields=dict(line.split(': ',1) for line in text.splitlines() if ': ' in line)
    if (fields.get('release'),fields.get('commit-hash'),fields.get('host')) != (manifest['rust_release'],manifest['rust_commit'],manifest['host']):
        raise ValueError('compiler identity does not match reviewed notice inputs')

def compiler_context(project_dir, manifest, run=None):
    """Resolve once, check in Cargo's directory, and force that compiler for metadata."""
    if run is None: run=subprocess.check_output
    compiler=Path(run(['rustup','which','--toolchain',TOOLCHAIN,'rustc'],cwd=project_dir,text=True).strip())
    if not compiler.is_absolute() or not compiler.is_file(): raise ValueError('resolved compiler is not an absolute file')
    identity=run([str(compiler),'-Vv'],cwd=project_dir,text=True)
    verify_compiler(identity,manifest)
    env=os.environ.copy(); env['RUSTC']=str(compiler); env['RUSTC_WRAPPER']=''; env['RUSTC_WORKSPACE_WRAPPER']=''
    evidence={'toolchain':TOOLCHAIN,'rustc_vv':identity,'rustc_sha256':sha(compiler.read_bytes()),'note':'Compiler identity for metadata only; does not attest the supplied binary.'}
    return env,evidence

def current_metadata(project_dir, env, run=None):
    if run is None: run=subprocess.check_output
    return json.loads(run(['cargo','+'+TOOLCHAIN,'metadata','--locked','--filter-platform',TARGET,'--format-version','1'],cwd=project_dir,env=env,text=True))

def verify_inventory(metadata, index, text):
    a={(p['name'],p['version'],p['license']) for p in metadata['packages']}
    b={(p['name'],p['version'],p['license']) for p in index['packages']}
    if not a or a!=b or index.get('missing_notice_files') or any(not p['notices'] for p in index['packages']):
        raise ValueError('metadata and complete notice inventory do not match')
    if index.get('bundle_sha256')!=sha(text): raise ValueError('notice text hash mismatch')
    if any(any(t in (lic or '').upper() for t in ('LGPL','GPL-3','MPL')) for _,_,lic in a):
        raise ValueError('selected metadata contains an excluded license expression; review required')

def check_name(name):
    p=PurePosixPath(name)
    if not name or p.is_absolute() or '\\' in name or ':' in name or any(x in ('..','.') for x in p.parts):
        raise ValueError('unsafe archive path: '+name)

def verified_inputs(manifest, root, game):
    result={}
    if not manifest.get('files'): raise ValueError('empty supplement manifest')
    for name,digest in manifest['files'].items():
        check_name(name)
        if name.startswith('unicode/') and game!='ratatui': continue
        value=(root/name).read_bytes()
        if sha(value)!=digest: raise ValueError('supplement hash mismatch: '+name)
        result['notices/'+name]=value
    return result

def scan(data, forbidden):
    for name,content in data.items():
        check_name(name)
        for term in forbidden:
            b=term.lower().encode('utf8')
            if b in content.lower() or term.lower().encode('utf-16le') in content.lower():
                raise ValueError('private path marker found in '+name)

def make_zip(path, data, forbidden):
    if 'SHA256SUMS' in data: raise ValueError('reserved checksum file')
    scan(data,forbidden)
    data=dict(data); data['SHA256SUMS']=''.join(f'{sha(v)}  {n}\n' for n,v in sorted(data.items())).encode()
    with zipfile.ZipFile(path,'x',compression=zipfile.ZIP_DEFLATED,compresslevel=9) as z:
        for name,value in sorted(data.items()):
            i=zipfile.ZipInfo(name,(2020,1,1,0,0,0)); i.create_system=3; i.external_attr=0o100644<<16; i.compress_type=zipfile.ZIP_DEFLATED; z.writestr(i,value)
    with zipfile.ZipFile(path) as z:
        if z.testzip() is not None or any(z.read(n)!=v for n,v in data.items()): raise ValueError('ZIP verification failed')
    return sha(Path(path).read_bytes())

def main():
    p=argparse.ArgumentParser(); p.add_argument('--game',required=True,choices=('softbuffer','ratatui')); p.add_argument('--repo',required=True,type=Path); p.add_argument('--binary',required=True,type=Path); p.add_argument('--notice-index',required=True,type=Path); p.add_argument('--notice-text',required=True,type=Path); p.add_argument('--output',required=True,type=Path); p.add_argument('--target',required=True); p.add_argument('--forbid',action='append',required=True,help='specific workspace/home path or fragment to reject; repeat as needed'); args=p.parse_args()
    if args.target!=TARGET or any(len(x)<4 for x in args.forbid): raise ValueError('wrong target or insufficient privacy marker')
    manifest_bytes=(HERE/'inputs-manifest.json').read_bytes()
    if sha(manifest_bytes)!=MANIFEST_SHA256: raise ValueError('reviewed supplement manifest changed')
    manifest=json.loads(manifest_bytes)
    if manifest['target']!=TARGET: raise ValueError('supplement target mismatch')
    def git(*cmd): return subprocess.check_output(['git','-C',str(args.repo),*cmd],text=True).strip()
    if git('rev-parse','HEAD')!=COMMIT or git('rev-parse','HEAD^{tree}')!=TREE or git('status','--porcelain=v1','--untracked-files=no'):
        raise ValueError('repository is not the pinned commit and tree with clean tracked files')
    cwd=args.repo/'experiments'/args.game
    env,compiler_evidence=compiler_context(cwd,manifest)
    meta=current_metadata(cwd,env)
    text=args.notice_text.read_bytes(); index=json.loads(args.notice_index.read_text(encoding='utf-8-sig')); verify_inventory(meta,index,text)
    binary=args.binary.read_bytes(); verify_pe_x64(binary)
    data={f'pong-{args.game}.exe':binary,'LICENSE':(args.repo/'LICENSE').read_bytes(),'notices/dependencies.txt':text,'notices/dependencies-index.json':args.notice_index.read_bytes()}
    data.update(verified_inputs(manifest,HERE/'inputs',args.game))
    data['notices/inputs-manifest.json']=(HERE/'inputs-manifest.json').read_bytes()
    data['notices/compiler-context.json']=(json.dumps(compiler_evidence,indent=2)+'\n').encode()
    data['RUNNING.txt']=(f'Windows x64 experimental review build from commit {COMMIT}, tree {TREE}. Tracked files were clean; untracked and ignored files were not checked. Unsigned; compatible Microsoft Visual C++ v14 runtime/UCRT may be required. '+('Graphics backend and drivers may be required. ' if args.game=='softbuffer' else 'Requires a terminal at least 50 columns by 18 rows. ')+'Physical input and clean-machine behavior are unverified. The target-filtered notice graph includes build dependencies and does not prove final linkage. Compiler identity and checksums do not attest binary source or compilation. Review notices and runtime compatibility before distribution.\n').encode()
    forbidden=set(args.forbid); forbidden.update((str(args.repo.resolve()),str(args.repo.resolve()).replace('\\','/')))
    print(make_zip(args.output,data,forbidden))

if __name__=='__main__':
    try: main()
    except (ValueError,OSError,subprocess.CalledProcessError,KeyError) as exc:
        print('packaging failed:',exc,file=sys.stderr); sys.exit(1)
