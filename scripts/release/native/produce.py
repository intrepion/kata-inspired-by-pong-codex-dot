#!/usr/bin/env python3
"""Collect actual native build/test/replay evidence; not release qualification."""
import argparse
import hashlib
import json
import math
import os
import re
import subprocess
from pathlib import Path

SHA = re.compile(r'^[0-9a-f]{40}$')
PLANS = {
    ('ratatui', 'aarch64-unknown-linux-gnu'): ('experiments/ratatui/Cargo.toml', 'pong-ratatui'),
    ('ratatui', 'aarch64-apple-darwin'): ('experiments/ratatui/Cargo.toml', 'pong-ratatui'),
    ('ratatui', 'x86_64-pc-windows-msvc'): ('experiments/ratatui/Cargo.toml', 'pong-ratatui.exe'),
    ('softbuffer', 'x86_64-pc-windows-msvc'): ('experiments/softbuffer/Cargo.toml', 'pong-softbuffer.exe'),
    ('sdl3', 'aarch64-apple-darwin'): ('experiments/sdl3/Cargo.toml', 'pong-sdl3'),
}

class EvidenceError(Exception): pass


def sha256(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda: f.read(1024 * 1024), b''): h.update(block)
    return h.hexdigest()


def sysroot_digest(sysroot, target):
    """Hash the selected target lib directory; this is not all compiler inputs."""
    directory = Path(sysroot) / 'lib' / 'rustlib' / target / 'lib'
    if not directory.is_dir() or directory.is_symlink(): raise EvidenceError('missing target sysroot libraries')
    files = sorted(directory.iterdir(), key=lambda p: p.name)
    if not files: raise EvidenceError('empty target sysroot libraries')
    h = hashlib.sha256()
    for path in files:
        if path.is_symlink() or not path.is_file(): raise EvidenceError('non-regular target sysroot entry')
        h.update((path.name + '\0' + sha256(path) + '\n').encode())
    return h.hexdigest()


def compiler_info(text):
    commit = re.search(r'^commit-hash: ([0-9a-f]{40})$', text, re.M)
    host = re.search(r'^host: ([A-Za-z0-9_-]+)$', text, re.M)
    if not commit or not host: raise EvidenceError('rustc verbose identity incomplete')
    return commit.group(1), host.group(1)


def verify_replay(text, reference):
    try:
        actual = [json.loads(line) for line in text.splitlines() if line.strip()]
        expected = [json.loads(line) for line in reference.splitlines() if line.strip()]
        if len(actual) != 13 or len(expected) != 13: raise EvidenceError('replay must have 13 records')
        for index, (a, e) in enumerate(zip(actual, expected)):
            if not isinstance(a, dict) or not isinstance(e, dict): raise EvidenceError('replay record is not an object')
            for key in ('scenario', 'seed', 'tick', 'state', 'scores', 'finished'):
                if type(a.get(key)) is not type(e.get(key)) or a.get(key) != e.get(key): raise EvidenceError(f'replay {key} mismatch')
            if a['tick'] != index * 120 or not isinstance(a.get('ball'), list) or len(a['ball']) != 4:
                raise EvidenceError('replay tick or ball shape mismatch')
            for x, y in zip([a.get('player'), a.get('cpu'), *a['ball']], [e.get('player'), e.get('cpu'), *e['ball']]):
                if isinstance(x, bool) or not isinstance(x, (float, int)) or not math.isfinite(x) or abs(x-y) > 0.001:
                    raise EvidenceError('replay numeric mismatch')
        if actual[-1].get('finished') is not True: raise EvidenceError('replay incomplete')
    except (ValueError, TypeError, KeyError, json.JSONDecodeError) as error:
        raise EvidenceError('invalid replay JSON or numeric field') from error


def subprocess_runner(argv, cwd, env, timeout):
    try:
        result = subprocess.run(argv, cwd=cwd, env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout, check=False)
    except (OSError, subprocess.TimeoutExpired) as error:
        raise EvidenceError(f'command failed or timed out: {Path(argv[0]).name}') from error
    if result.returncode:
        raise EvidenceError(f'command returned {result.returncode}: {Path(argv[0]).name}\n{result.stderr[-4000:]}')
    return result.stdout, result.stderr


def collect(repo, output, package, target, expected_sha, expected_tree, runner=subprocess_runner):
    if not SHA.fullmatch(expected_sha) or not SHA.fullmatch(expected_tree): raise EvidenceError('expected commit and tree must be full SHA-1 identifiers')
    if (package, target) not in PLANS: raise EvidenceError('package/target not in initial scope')
    repo = Path(repo).resolve(strict=True)
    requested_output = Path(output)
    output = requested_output.parent.resolve(strict=True) / requested_output.name
    if output.exists() or repo == output or repo in output.parents: raise EvidenceError('output must be new and outside checkout')
    env = os.environ.copy()
    for key in list(env):
        if key in {'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTC', 'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER'} or key.startswith(('CARGO_BUILD_', 'CARGO_TARGET_')):
            env.pop(key, None)
    # Explicit empties override Cargo config, unlike merely removing the variables.
    env['RUSTC_WRAPPER'] = ''; env['RUSTC_WORKSPACE_WRAPPER'] = ''
    def call(argv, timeout=60): return runner([str(x) for x in argv], repo, env, timeout)
    def clean_source():
        top, _ = call(['git', 'rev-parse', '--show-toplevel'])
        if Path(top.strip()).resolve() != repo: raise EvidenceError('repo must be the checkout root')
        head, _ = call(['git', 'rev-parse', 'HEAD']); tree, _ = call(['git', 'rev-parse', 'HEAD^{tree}'])
        status, _ = call(['git', 'status', '--porcelain=v1', '--untracked-files=all', '--ignored=matching'])
        if head.strip() != expected_sha or tree.strip() != expected_tree: raise EvidenceError('source commit/tree mismatch')
        if status.strip(): raise EvidenceError('checkout is not clean, including ignored files')
    clean_source()
    sysroot_text, _ = call(['rustc', '--print', 'sysroot']); sysroot = Path(sysroot_text.strip()).resolve(strict=True)
    suffix = '.exe' if target.endswith('-windows-msvc') else ''
    rustc = sysroot / 'bin' / ('rustc' + suffix)
    if not rustc.is_file(): raise EvidenceError('selected sysroot compiler is missing')
    version, _ = call([rustc, '-vV']); commit, host = compiler_info(version)
    if host != target: raise EvidenceError('cross-target build/replay is not supported by this producer')
    cargo = sysroot / 'bin' / ('cargo' + suffix)
    if not cargo.is_file(): raise EvidenceError('selected sysroot cargo is missing')
    cargo_version, _ = call([cargo, '--version'])
    rustc_hash = sha256(rustc); cargo_hash = sha256(cargo); libs_hash = sysroot_digest(sysroot, target)
    output.mkdir(parents=True, exist_ok=False)
    env['RUSTC'] = str(rustc); env['CARGO_TARGET_DIR'] = str(output / 'target'); env['CARGO_NET_OFFLINE'] = 'true'
    env['CARGO_ENCODED_RUSTFLAGS'] = '--remap-path-prefix=' + str(Path.home()) + '=/user\x1f--remap-path-prefix=' + str(repo) + '=/src'
    manifest, exe = PLANS[(package, target)]
    manifest_path = repo / manifest
    if not manifest_path.is_file() or manifest_path.is_symlink(): raise EvidenceError('package manifest missing or unsafe')
    base = [str(cargo), '--offline', '--locked', '--release', '--target', target, '--manifest-path', str(manifest_path)]
    build_out, build_err = call([base[0], 'build', *base[1:]], 1800)
    (output / 'build.log').write_text(build_out + build_err)
    binary = output / 'target' / target / 'release' / exe
    if binary.is_symlink() or not binary.is_file() or not binary.stat().st_size: raise EvidenceError('fresh output binary missing')
    binary_hash = sha256(binary)
    # Compile tests without Cargo executing any configured target runner. Only
    # the named adapter unit-test executable inside this fresh target is run.
    test_build_out, test_build_err = call([base[0], 'test', *base[1:], '--no-run', '--message-format=json'], 1800)
    (output / 'test-build.jsonl').write_text(test_build_out)
    (output / 'test-build.stderr').write_text(test_build_err)
    adapter_name = exe.removesuffix('.exe')
    candidates = []
    try:
        for line in test_build_out.splitlines():
            message = json.loads(line)
            if (message.get('reason') == 'compiler-artifact' and message.get('profile', {}).get('test') is True
                    and message.get('target', {}).get('name') == adapter_name
                    and message.get('target', {}).get('kind') == ['bin'] and message.get('executable')):
                candidates.append(Path(message['executable']))
    except (ValueError, TypeError, AttributeError) as error:
        raise EvidenceError('invalid Cargo test artifact JSON') from error
    if len(candidates) != 1: raise EvidenceError('expected one adapter test executable')
    test_binary = candidates[0]
    target_root = (output / 'target').resolve(strict=True)
    if not test_binary.is_absolute() or test_binary.is_symlink() or not test_binary.is_file() or not test_binary.resolve().is_relative_to(target_root):
        raise EvidenceError('adapter test executable outside fresh target')
    if target.endswith('-windows-msvc') and test_binary.suffix.lower() != '.exe':
        raise EvidenceError('Windows adapter test executable must end in .exe')
    test_binary_hash = sha256(test_binary)
    test_out, test_err = call([test_binary, '--nocapture'], 1800)
    (output / 'tests.log').write_text(test_out + test_err)
    counts = re.findall(r'test result: ok\. (\d+) passed;', test_out + test_err)
    if not counts or sum(map(int, counts)) == 0: raise EvidenceError('no passing native adapter tests observed')
    if sha256(test_binary) != test_binary_hash: raise EvidenceError('test binary changed during execution')
    replay_out, replay_err = call([binary, '--scripted-test'], 90)
    (output / 'replay.jsonl').write_text(replay_out); (output / 'replay.stderr').write_text(replay_err)
    verify_replay(replay_out, (repo / 'docs/evidence/rust-core-contact-timing-model-20261112.jsonl').read_text())
    clean_source()
    if sha256(binary) != binary_hash or sha256(rustc) != rustc_hash or sha256(cargo) != cargo_hash or sysroot_digest(sysroot, target) != libs_hash:
        raise EvidenceError('binary or compiler/sysroot changed during evidence collection')
    record = {'schema': 1, 'package': package, 'source_sha': expected_sha, 'tree_sha': expected_tree, 'target': target,
              'rustc_commit': commit, 'rustc_sha256': rustc_hash, 'cargo_sha256': cargo_hash, 'target_sysroot_sha256': libs_hash,
              'rustc_verbose': version, 'cargo_version': cargo_version.strip(), 'binary_sha256': binary_hash, 'test_binary_sha256': test_binary_hash,
              'test_binary_relative_path': str(test_binary.relative_to(output)).replace(os.sep, '/'),
              'binary_relative_path': str(binary.relative_to(output)).replace(os.sep, '/'),
              'evidence': {name: sha256(output / name) for name in ('build.log', 'test-build.jsonl', 'test-build.stderr', 'tests.log', 'replay.jsonl', 'replay.stderr')},
              'limitations': 'Only fresh build, direct adapter binary unit tests and app-owned replay are recorded; integration and doc tests are not recorded. This is not a notice, privacy, package, physical-input, clean-machine, signing or release decision.'}
    (output / 'build-evidence.json').write_text(json.dumps(record, indent=2, sort_keys=True) + '\n')
    return record


def main():
    p=argparse.ArgumentParser(description=__doc__)
    for name in ('repo','output','package','target','expected-sha','expected-tree'): p.add_argument('--'+name, required=True)
    a=p.parse_args()
    try: record=collect(a.repo,a.output,a.package,a.target,a.expected_sha,a.expected_tree)
    except (EvidenceError,OSError) as error: p.exit(1, f'native evidence: {error}\n')
    print(json.dumps({'binary_sha256':record['binary_sha256'],'output':a.output,'notice_review':'not performed'}))

if __name__ == '__main__': main()
