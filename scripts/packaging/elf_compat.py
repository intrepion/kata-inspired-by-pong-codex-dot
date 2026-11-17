#!/usr/bin/env python3
"""Report ELF runtime requirements using readelf, without executing the input."""
import argparse
import hashlib
import json
import os
import re
import stat
import subprocess
from pathlib import Path

VERSION = re.compile(r'^(GLIBC|GLIBCXX|CXXABI)_(\d+(?:\.\d+)+)$')
SPECIAL = re.compile(r'^(GLIBC|GLIBCXX|CXXABI)_[A-Za-z0-9_]+$')
SHA1 = re.compile(r'^[0-9a-f]{40}$')

class ReportError(Exception): pass


def parse_readelf(text):
    def header(name):
        found = re.findall(r'^\s*' + re.escape(name) + r':\s*(.+?)\s*$', text, re.M)
        if len(found) != 1: raise ReportError('missing or ambiguous ELF header: ' + name)
        return found[0]
    if not text.startswith('ELF Header:'): raise ReportError('readelf did not return an ELF header')
    result = {'class':header('Class'), 'data':header('Data'), 'type':header('Type'), 'machine':header('Machine')}
    interpreters = re.findall(r'\[Requesting program interpreter:\s*([^\]]+)\]', text)
    if len(interpreters)>1: raise ReportError('multiple ELF interpreters')
    result['interpreter'] = interpreters[0] if interpreters else None
    result['needed'] = re.findall(r'\(NEEDED\).*?Shared library:\s*\[([^\]]+)\]', text)
    heading_pattern = r"(?m)^Version (?:symbols|definition|needs) section\b[^\n]*\n"
    sections = re.split(heading_pattern, text)
    headings = re.findall(heading_pattern, text)
    needs = [i for i,h in enumerate(headings) if h.startswith('Version needs section')]
    # Distinguish a documented absence from an incomplete or unknown readelf format.
    if len(needs) > 1 or text.count('Version needs section') != len(needs):
        raise ReportError('ambiguous or unrecognized version-needs section')
    if not headings and 'No version information found in this file.' not in text:
        raise ReportError('missing or unrecognized version information')
    result['version_needs_section_present'] = bool(needs)
    verneed = re.findall(r'\(VERNEED\)\s+0x[0-9a-fA-F]+\b', text)
    verneednum = re.findall(r'\(VERNEEDNUM\)\s+(\d+)\b', text)
    if text.count('(VERNEED)') != len(verneed) or text.count('(VERNEEDNUM)') != len(verneednum) or len(verneed) > 1 or len(verneednum) > 1:
        raise ReportError('unrecognized or duplicate dynamic version-needs tags')
    if bool(verneed) != bool(verneednum) or (verneed and not needs):
        raise ReportError('dynamic version-needs tags and section disagree')
    if needs and result['type'].startswith(('DYN', 'EXEC')) and not verneed:
        raise ReportError('version-needs section lacks dynamic version-needs tags')
    result['version_requirements_status'] = ('validated_needs_section' if needs else 'no_needs_section_reported_not_compatibility_proof')
    required = {'GLIBC':set(), 'GLIBCXX':set(), 'CXXABI':set()}; special=set(); by_library={}
    if needs:
        i=needs[0]; heading=headings[i]; body=sections[i+1]
        count=re.fullmatch(r"Version needs section '[^']+' contains (\d+) entr(?:y|ies):\n",heading)
        if not count: raise ReportError('unrecognized version-needs heading')
        expected_files=int(count.group(1)); files=0; library=None; remaining=0; address_seen=False
        if verneednum and int(verneednum[0]) != expected_files:
            raise ReportError('dynamic version-needs count mismatch')
        for line in body.splitlines():
            line=line.strip()
            if not line: continue
            if re.fullmatch(r'Addr:\s*0x[0-9a-fA-F]+\s+Offset:\s*0x[0-9a-fA-F]+\s+Link:\s*\d+(?:\s+\([^)]*\))?',line):
                if address_seen or files: raise ReportError('unexpected version-needs address record')
                address_seen=True; continue
            f=re.fullmatch(r'(?:0x)?[0-9a-fA-F]+:\s*Version:\s*(\d+)\s+File:\s*(\S+)\s+Cnt:\s*(\d+)',line)
            if f:
                if not address_seen or remaining or f.group(1) != '1': raise ReportError('invalid or truncated version-needs library record')
                library=f.group(2); remaining=int(f.group(3)); files+=1
                if remaining <= 0: raise ReportError('empty version-needs library record')
                continue
            m=re.fullmatch(r'(?:0x)?[0-9a-fA-F]+:\s*Name:\s*(\S+)\s+Flags:\s*.+?\s+Version:\s*\d+',line)
            if not m or library is None or remaining <= 0: raise ReportError('unrecognized or extra version-needs record')
            remaining-=1; value=m.group(1); v=VERSION.fullmatch(value)
            if v:
                required[v.group(1)].add(value);by_library.setdefault(library,set()).add(value)
            elif SPECIAL.fullmatch(value):
                special.add(value);by_library.setdefault(library,set()).add(value)
            elif value.startswith(('GLIBC_','GLIBCXX_','CXXABI_')):
                raise ReportError('unrecognized compatibility version node')
        if not address_seen or remaining or files != expected_files:
            raise ReportError('truncated version-needs section or count mismatch')
    def order(v): return tuple(map(int,v.split('_',1)[1].split('.')))
    result['version_requirements'] = {k:sorted(v,key=order) for k,v in required.items()}
    result['minimum_symbol_versions'] = {k:(max(v,key=order) if v else None) for k,v in required.items()}
    result['special_version_requirements'] = sorted(special)
    result['requirements_by_library'] = {k:sorted(v) for k,v in sorted(by_library.items())}
    return result


def digest_fd(fd):
    os.lseek(fd,0,os.SEEK_SET); h=hashlib.sha256()
    while chunk := os.read(fd,1024*1024): h.update(chunk)
    return h.hexdigest()


def inspect(path, source_revision=None):
    if source_revision is not None and not SHA1.fullmatch(source_revision): raise ReportError('source revision must be a full lowercase SHA-1')
    flags=os.O_RDONLY | getattr(os,'O_CLOEXEC',0) | getattr(os,'O_NOFOLLOW',0) | getattr(os,'O_NONBLOCK',0)
    try: fd=os.open(path,flags)
    except OSError as e: raise ReportError('cannot open input as a regular non-symlink file') from e
    try:
        if not stat.S_ISREG(os.fstat(fd).st_mode): raise ReportError('input is not a regular file')
        before=digest_fd(fd)
        # The descriptor is inherited only by readelf; no input executable is run.
        try:
            env=os.environ.copy();env.update({'LC_ALL':'C','LANG':'C'})
            result=subprocess.run(['/usr/bin/readelf','-W','-h','-l','-d','-V',f'/proc/self/fd/{fd}'],pass_fds=(fd,),capture_output=True,text=True,timeout=30,check=False,env=env)
        except (OSError,subprocess.TimeoutExpired) as e: raise ReportError('readelf unavailable or timed out') from e
        if result.returncode: raise ReportError('readelf rejected the input')
        if result.stderr.strip(): raise ReportError('readelf produced diagnostic output; report is not trusted')
        after=digest_fd(fd)
        if before != after: raise ReportError('input changed during inspection')
        info=parse_readelf(result.stdout)
        info.update({'schema':1,'input_name':Path(path).name,'sha256':before,'source_revision_claim':source_revision,
                     'source_revision_verified':False,'method':'readelf static inspection; input not executed',
                     'limitations':['Symbol versions are not a complete OS, kernel, CPU, graphics or clean-machine compatibility guarantee.',
                                    'DT_NEEDED omits libraries loaded dynamically at runtime and transitive system dependencies.',
                                    'The source revision is caller-supplied and not verified by this report.',
                                    'This report does not assess licenses, bundled components or legal clearance.']})
        return info
    finally: os.close(fd)


def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('elf');p.add_argument('--source-revision')
    args=p.parse_args()
    try: report=inspect(args.elf,args.source_revision)
    except (ReportError,OSError,UnicodeError) as e: p.exit(1,f'ELF report: {e}\n')
    print(json.dumps(report,indent=2,sort_keys=True))

if __name__=='__main__': main()
