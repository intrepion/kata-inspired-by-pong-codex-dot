"""Ratatui-only proposed bridge; packaging remains disabled by the notice gate."""
import argparse, os, sys
from pathlib import Path
local_producer=Path(__file__).resolve().parent/'producer'
if local_producer.is_dir(): sys.path.insert(0,str(local_producer))
from produce import collect, EvidenceError
import handoff

def run(repo,evidence,output,producer=collect,consumer=handoff.attempt_package):
    previous=os.environ.get('RUSTUP_TOOLCHAIN')
    os.environ['RUSTUP_TOOLCHAIN']='stable-x86_64-pc-windows-msvc'
    try:
        producer(repo,evidence,'ratatui',handoff.TARGET,handoff.SOURCE,handoff.TREE)
        return consumer(repo,evidence,output,lambda binary,dest: (_ for _ in ()).throw(handoff.HandoffError('packager not yet wired')))
    finally:
        if previous is None: os.environ.pop('RUSTUP_TOOLCHAIN',None)
        else: os.environ['RUSTUP_TOOLCHAIN']=previous

def main():
    p=argparse.ArgumentParser();p.add_argument('--repo',required=True,type=Path);p.add_argument('--evidence',required=True,type=Path);p.add_argument('--output',required=True,type=Path);a=p.parse_args()
    run(a.repo,a.evidence,a.output)

if __name__=='__main__':
    try: main()
    except (EvidenceError,handoff.HandoffError,OSError,ValueError) as error:
        print('pipeline stopped:',error,file=sys.stderr);sys.exit(1)
