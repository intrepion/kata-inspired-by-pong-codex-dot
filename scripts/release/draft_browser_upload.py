#!/usr/bin/env python3
"""Create one run-specific draft; never publish, update, overwrite or resume."""
import argparse
import hashlib
import io
import json
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
import zipfile
from pathlib import Path
from package_browser import FILES, ROOT

REPO = 'intrepion/kata-inspired-by-pong-codex-dot'
ARCHIVE = 'pong-typescript-browser.zip'
SUMS = 'SHA256SUMS'
MAX_ARCHIVE = 5 * 1024 * 1024
MAX_UNPACKED = 10 * 1024 * 1024
SHA = re.compile(r'^[0-9a-f]{40}$')
DIGEST = re.compile(r'^[0-9a-f]{64}$')
NUMBER = re.compile(r'^[1-9][0-9]{0,19}$')

class Failure(Exception): pass


def inspect_artifacts(directory, source_sha, expected_digest):
    directory = Path(directory)
    if directory.is_symlink() or not directory.is_dir(): raise Failure('artifact directory invalid')
    files = list(directory.iterdir())
    if {p.name for p in files} != {ARCHIVE, SUMS} or len(files) != 2: raise Failure('unexpected artifact files')
    if any(p.is_symlink() or not p.is_file() for p in files): raise Failure('invalid artifact file')
    archive, sums = directory / ARCHIVE, directory / SUMS
    if archive.stat().st_size > MAX_ARCHIVE or sums.stat().st_size > 256: raise Failure('artifact too large')
    payload = archive.read_bytes(); digest = hashlib.sha256(payload).hexdigest()
    if not DIGEST.fullmatch(expected_digest) or digest != expected_digest: raise Failure('artifact digest mismatch')
    if sums.read_bytes() != f'{digest}  {ARCHIVE}\n'.encode(): raise Failure('external checksum mismatch')
    expected = {ROOT + name for name in (*FILES, 'README.txt', 'SHA256SUMS')}
    try:
        with zipfile.ZipFile(io.BytesIO(payload)) as z:
            infos = z.infolist(); names = [x.filename for x in infos]
            if len(names) != len(expected) or len(set(names)) != len(names) or set(names) != expected: raise Failure('unexpected archive members')
            if sum(x.file_size for x in infos) > MAX_UNPACKED: raise Failure('archive content too large')
            if any(x.is_dir() or (x.external_attr >> 16) & 0o170000 != 0o100000 for x in infos): raise Failure('unsafe archive member type')
            data = {x.filename: z.read(x) for x in infos}
    except (zipfile.BadZipFile, OSError, RuntimeError, ValueError) as error:
        raise Failure('invalid archive') from error
    manifest = data[ROOT + 'SHA256SUMS']
    expected_internal = ''.join(f'{hashlib.sha256(data[ROOT + name]).hexdigest()}  ./{name}\n' for name in sorted((*FILES, 'README.txt'))).encode()
    if manifest != expected_internal: raise Failure('internal checksum mismatch')
    if f'Source commit: {source_sha}\n'.encode() not in data[ROOT + 'README.txt']: raise Failure('archive source mismatch')
    return [(ARCHIVE, payload, digest), (SUMS, sums.read_bytes(), hashlib.sha256(sums.read_bytes()).hexdigest())]


def ensure_release(release, release_id, tag, source_sha, expected_assets):
    if not isinstance(release, dict) or release.get('draft') is not True or release.get('prerelease') is not True or release.get('tag_name') != tag or release.get('target_commitish') != source_sha or release.get('id') != release_id:
        raise Failure('draft release state or target mismatch')
    assets = release.get('assets')
    if not isinstance(assets, list): raise Failure('release asset listing missing')
    actual = {}
    for asset in assets:
        if not isinstance(asset, dict) or not isinstance(asset.get('name'), str) or asset['name'] in actual:
            raise Failure('invalid or duplicate release assets')
        if asset.get('state') != 'uploaded': raise Failure('asset not fully uploaded')
        actual[asset['name']] = (asset.get('size'), asset.get('digest'))
    if actual != expected_assets: raise Failure('unexpected release assets or digest')


def create_draft(transport, source_sha, run_id, attempt, artifacts, expected_digest):
    if not SHA.fullmatch(source_sha) or not NUMBER.fullmatch(run_id) or not NUMBER.fullmatch(attempt): raise Failure('invalid trusted run identity')
    verified = inspect_artifacts(artifacts, source_sha, expected_digest)  # no network writes before complete validation
    tag = f'draft-pong-{source_sha}-{run_id}-{attempt}'
    # Authenticated paginated list also sees drafts; a tag lookup alone may not.
    for page in range(1, 101):
        releases = transport.list_releases(page)
        if not isinstance(releases, list): raise Failure('invalid release list')
        if any(not isinstance(r, dict) or not isinstance(r.get('tag_name'), str) or r['tag_name'] == tag for r in releases): raise Failure('release tag already exists or list invalid')
        if len(releases) < 100: break
    else: raise Failure('release listing limit reached')
    if transport.tag_exists(tag): raise Failure('git tag already exists')
    created = transport.create(tag, source_sha)
    if not isinstance(created, dict) or type(created.get('id')) is not int or created['id'] <= 0: raise Failure('invalid create response; do not retry')
    release_id = created['id']; expected = {}
    ensure_release(created, release_id, tag, source_sha, expected)
    for name, content, digest in verified:
        ensure_release(transport.get(release_id), release_id, tag, source_sha, expected)
        transport.upload(release_id, name, content)  # uncertain failure must stop; never retry
        expected[name] = (len(content), 'sha256:' + digest)
        ensure_release(transport.get(release_id), release_id, tag, source_sha, expected)
    return release_id


class Transport:
    def __init__(self, token):
        if not token: raise Failure('missing uploader token')
        self.token = token
        class NoRedirect(urllib.request.HTTPRedirectHandler):
            def redirect_request(self, *args, **kwargs): return None
        self.opener = urllib.request.build_opener(NoRedirect())
    def request(self, host, path, method='GET', body=None, content_type='application/vnd.github+json', allow_404=False):
        if host not in ('api.github.com', 'uploads.github.com'): raise Failure('invalid API host')
        req = urllib.request.Request('https://' + host + path, data=body, method=method, headers={'Authorization':'Bearer '+self.token,'Accept':'application/vnd.github+json','Content-Type':content_type,'X-GitHub-Api-Version':'2022-11-28','User-Agent':'pong-draft-uploader'})
        try:
            with self.opener.open(req, timeout=30) as response: return json.load(response)
        except urllib.error.HTTPError as e:
            if allow_404 and e.code == 404: return None
            raise Failure(f'GitHub request failed with HTTP {e.code}; do not retry uncertain writes') from e
        except (urllib.error.URLError, OSError, json.JSONDecodeError) as e:
            raise Failure('GitHub request failed or response invalid; do not retry uncertain writes') from e
    def list_releases(self,page): return self.request('api.github.com',f'/repos/{REPO}/releases?per_page=100&page={page}')
    def tag_exists(self,tag): return self.request('api.github.com',f'/repos/{REPO}/git/ref/tags/{urllib.parse.quote(tag,safe="")}',allow_404=True) is not None
    def create(self,tag,sha):
        body=json.dumps({'tag_name':tag,'target_commitish':sha,'name':'Pong browser draft '+sha[:8], 'body':'Automated browser-only draft. No public release or compatibility/license clearance is implied.', 'draft':True,'prerelease':True}).encode()
        return self.request('api.github.com',f'/repos/{REPO}/releases','POST',body)
    def get(self,release_id): return self.request('api.github.com',f'/repos/{REPO}/releases/{release_id}')
    def upload(self,release_id,name,data):
        if name not in (ARCHIVE,SUMS): raise Failure('unexpected upload name')
        return self.request('uploads.github.com',f'/repos/{REPO}/releases/{release_id}/assets?name={urllib.parse.quote(name,safe="")}', 'POST',data,'application/octet-stream')


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--artifacts',type=Path,required=True);p.add_argument('--sha',required=True);p.add_argument('--run-id',required=True);p.add_argument('--attempt',required=True);p.add_argument('--expected-digest',required=True)
    a=p.parse_args()
    try:
        if os.environ.get('GITHUB_REPOSITORY') != REPO: raise Failure('repository mismatch')
        if (os.environ.get('GITHUB_SHA'), os.environ.get('GITHUB_RUN_ID'), os.environ.get('GITHUB_RUN_ATTEMPT')) != (a.sha, a.run_id, a.attempt): raise Failure('current run identity mismatch')
        if os.environ.get('GITHUB_REF') != 'refs/heads/main' or os.environ.get('GITHUB_EVENT_NAME') != 'workflow_dispatch': raise Failure('untrusted trigger')
        rid=create_draft(Transport(os.environ.get('GH_TOKEN')),a.sha,a.run_id,a.attempt,a.artifacts,a.expected_digest)
    except Failure as e:
        p.exit(1,f'draft uploader: {e}\n')
    print(f'Created draft release id {rid}; do not publish during or before independent review.')

if __name__ == '__main__': main()
