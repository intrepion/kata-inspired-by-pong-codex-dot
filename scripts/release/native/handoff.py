"""Ratatui producer-to-packager handoff. No release ZIP until notice binding exists."""
import argparse, hashlib, json, re, subprocess, sys
from pathlib import Path, PurePosixPath

SOURCE='d72c7f6a8fc8a10cd1a8a8bdbb7fa131085bf4aa'
TREE='8cb5b0ddaa4b34f48e162a66114f5472b7502828'
TARGET='x86_64-pc-windows-msvc'
RUST='58cf8f9c35b355199727627d07ae34a4e2846d67'
LOGS={'build.log','test-build.jsonl','test-build.stderr','tests.log','replay.jsonl','replay.stderr'}
HEX=re.compile(r'^[0-9a-f]{64}$')

class HandoffError(ValueError): pass

def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()

def safe_file(root, relative):
    p=PurePosixPath(relative)
    if not relative or p.is_absolute() or '\\' in relative or ':' in relative or any(x in ('.','..') for x in p.parts): raise HandoffError('unsafe evidence path')
    path=root
    for part in p.parts:
        path=path/part
        if path.is_symlink() or (hasattr(path,'is_junction') and path.is_junction()): raise HandoffError('linked evidence path')
    if not path.is_file() or not path.resolve().is_relative_to(root.resolve()): raise HandoffError('missing or outside evidence file')
    return path

def check_source(repo, run=subprocess.check_output):
    def git(*a): return run(['git','-C',str(repo),*a],text=True).strip()
    if git('rev-parse','HEAD')!=SOURCE or git('rev-parse','HEAD^{tree}')!=TREE or git('status','--porcelain=v1','--untracked-files=all','--ignored'):
        raise HandoffError('source commit/tree mismatch or checkout not fully clean')

def verify_evidence(root):
    root=Path(root).resolve(strict=True)
    record=json.loads(safe_file(root,'build-evidence.json').read_text(encoding='utf8'))
    if (record.get('schema'),record.get('package'),record.get('source_sha'),record.get('tree_sha'),record.get('target'),record.get('rustc_commit'))!=(1,'ratatui',SOURCE,TREE,TARGET,RUST):
        raise HandoffError('producer evidence identity mismatch')
    if record.get('binary_relative_path')!='target/'+TARGET+'/release/pong-ratatui.exe': raise HandoffError('unexpected binary location')
    logs=record.get('evidence',{})
    if set(logs)!=LOGS: raise HandoffError('incomplete producer evidence')
    pairs=list(logs.items())+[(record['binary_relative_path'],record.get('binary_sha256','')),(record.get('test_binary_relative_path',''),record.get('test_binary_sha256',''))]
    test_prefix='target/'+TARGET+'/release/deps/'
    if not record.get('test_binary_relative_path','').startswith(test_prefix): raise HandoffError('unexpected test binary location')
    for name,expected in pairs:
        if not isinstance(expected,str) or not HEX.fullmatch(expected) or digest(safe_file(root,name))!=expected: raise HandoffError('evidence hash mismatch: '+name)
    for name in ('rustc_sha256','cargo_sha256','target_sysroot_sha256'):
        if not isinstance(record.get(name),str) or not HEX.fullmatch(record[name]): raise HandoffError('missing toolchain digest')
    if not re.search(r'test result: ok\. [1-9][0-9]* passed;',safe_file(root,'tests.log').read_text(encoding='utf8')): raise HandoffError('no passing adapter tests')
    return record

def require_notice_binding(record):
    # Producer schema 1 captures neither locked graph nor authenticated notice inputs.
    # No caller flag, arbitrary index, or hash alone may waive this requirement.
    raise HandoffError('producer evidence lacks build-bound authenticated notice inputs; no package created')

def attempt_package(repo,evidence,output,packer):
    if Path(output).exists(): raise HandoffError('output must not exist')
    check_source(repo)
    record=verify_evidence(evidence)
    require_notice_binding(record)
    # Future implementations must pass only this verified producer output and
    # freshly authenticated notice inputs to the existing Windows packager.
    return packer(safe_file(Path(evidence).resolve(),record['binary_relative_path']),output)

def main():
    p=argparse.ArgumentParser();p.add_argument('--repo',required=True,type=Path);p.add_argument('--evidence',required=True,type=Path);p.add_argument('--output',required=True,type=Path);a=p.parse_args()
    attempt_package(a.repo,a.evidence,a.output,lambda binary,output: (_ for _ in ()).throw(HandoffError('packager not yet wired')))

if __name__=='__main__':
    try: main()
    except (HandoffError,OSError,KeyError,ValueError,subprocess.CalledProcessError) as error:
        print('handoff refused:',error,file=sys.stderr);sys.exit(1)
