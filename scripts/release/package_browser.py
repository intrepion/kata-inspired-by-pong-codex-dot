#!/usr/bin/env python3
"""Package the first-party browser client; this does not create a release."""
import argparse
import hashlib
import os
import re
import zipfile
from pathlib import Path

FILES = ('LICENSE', 'index.html', 'style.css', 'game.js', 'scenarios/smoke-v1.txt')
ROOT = 'pong-typescript-browser/'


def private_path_patterns(source: Path, output: Path) -> set[bytes]:
    # Include actual runner paths and their two-component suffixes, which can
    # survive an incomplete remap. Do not print these values in failure logs.
    values = [str(source), str(output.resolve()), str(output.resolve().parent)]
    values.extend(os.environ.get(name, '') for name in ('GITHUB_WORKSPACE', 'HOME', 'USERPROFILE', 'RUNNER_TEMP'))
    variants = {'/home/runner/', '/Users/runner/', 'C:\\Users\\runner\\'}
    for value in values:
        if not value or len(value) < 6:
            continue
        normalized = value.replace('\\', '/')
        if not (normalized.startswith('/') or re.match(r'^[A-Za-z]:/', normalized)):
            continue
        parts = [part for part in normalized.split('/') if part]
        candidates = [normalized.rstrip('/')]
        if len(parts) >= 2:
            suffix = '/'.join(parts[-2:])
            if len(suffix) >= 12:
                candidates.append(suffix)
        for candidate in candidates:
            variants.add(candidate)
            variants.add(candidate.replace('/', '\\'))
    return {text.encode(encoding) for text in variants for encoding in ('utf-8', 'utf-16le', 'utf-16be')}


def package(source: Path, output: Path, sha: str) -> None:
    if not re.fullmatch(r'[0-9a-f]{40}', sha):
        raise ValueError('source revision must be a full lowercase commit SHA')
    source = source.resolve(strict=True)
    if output.exists():
        raise ValueError('output already exists')
    entries = {}
    for name in FILES:
        path = source / name
        if path.is_symlink() or not path.is_file() or source not in path.resolve().parents:
            raise ValueError(f'unsafe or missing source: {name}')
        entries[name] = path.read_bytes()
    readme = (f'Pong TypeScript browser — experimental build.\nSource commit: {sha}\n'
              'Serve over HTTP, for example: python3 -m http.server 8000. '
              'Open http://localhost:8000/; do not use a file URL.\n'
              'Automated checks do not establish browser rendering, physical input '
              'or clean-machine compatibility. Retain LICENSE.\n')
    entries['README.txt'] = readme.encode()
    prohibited = private_path_patterns(source, output)
    for name, data in entries.items():
        if any(pattern in data for pattern in prohibited):
            raise ValueError(f'private build path in {name}')
    entries['SHA256SUMS'] = ''.join(f'{hashlib.sha256(data).hexdigest()}  ./{name}\n' for name, data in sorted(entries.items())).encode()
    output.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(output, 'x', compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for name, data in sorted(entries.items()):
            info = zipfile.ZipInfo(ROOT + name, (2020, 1, 1, 0, 0, 0))
            info.create_system = 3
            info.external_attr = 0o100644 << 16
            info.compress_type = zipfile.ZIP_DEFLATED
            archive.writestr(info, data)
    with zipfile.ZipFile(output) as archive:
        if archive.testzip() or set(archive.namelist()) != {ROOT + name for name in entries}:
            raise ValueError('archive validation failed')
        for name, expected in entries.items():
            if archive.read(ROOT + name) != expected:
                raise ValueError(f'archive content mismatch: {name}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--source', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--sha', required=True)
    args = parser.parse_args()
    package(args.source, args.output, args.sha)
