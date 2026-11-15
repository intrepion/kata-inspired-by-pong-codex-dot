// Verify the supplied source-file inventory without claiming it authenticates
// its own base-commit assertion; a reviewer must compare it with Git.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const digest = (algorithm, bytes) => crypto.createHash(algorithm).update(bytes).digest('hex');
function verifySourceManifest(sourceRoot, manifestFile) {
  const root = fs.realpathSync(sourceRoot);
  const file = path.resolve(manifestFile);
  if (fs.lstatSync(file).isSymbolicLink() || fs.realpathSync(file) !== file) throw Error('Source manifest symlink refused');
  const bytes = fs.readFileSync(file);
  const manifest = JSON.parse(bytes.toString('utf8'));
  if (!manifest || typeof manifest !== 'object' || Array.isArray(manifest) ||
      Object.keys(manifest).some(k => !['schema','base_commit','files','local_changes'].includes(k)) ||
      manifest.schema !== 1 || !/^[0-9a-f]{40}$/.test(manifest.base_commit) ||
      !Array.isArray(manifest.files) || !Array.isArray(manifest.local_changes))
    throw Error('Invalid source manifest');
  const names = new Set();
  for (const entry of manifest.files) {
    if (!entry || typeof entry !== 'object' || Array.isArray(entry) ||
        Object.keys(entry).some(k => !['path','git_blob','base_git_blob'].includes(k)) ||
        typeof entry.path !== 'string' || !entry.path || path.isAbsolute(entry.path) || /[\\\\:]/.test(entry.path) ||
        entry.path.split('/').some(x => !x || x === '.' || x === '..') ||
        !/^[0-9a-f]{40}$/.test(entry.git_blob) ||
        (entry.base_git_blob != null && !/^[0-9a-f]{40}$/.test(entry.base_git_blob)) || names.has(entry.path))
      throw Error('Invalid source manifest entry');
    names.add(entry.path);
    const target = path.resolve(root, entry.path);
    if (!target.startsWith(root+path.sep) || fs.lstatSync(target).isSymbolicLink() ||
        fs.realpathSync(target) !== target || !fs.lstatSync(target).isFile())
      throw Error('Invalid source file: '+entry.path);
    const content = fs.readFileSync(target);
    const header = Buffer.from('blob '+content.length+'\0');
    if (digest('sha1', Buffer.concat([header,content])) !== entry.git_blob)
      throw Error('Source hash mismatch: '+entry.path);
  }
  for (const required of ['LICENSE','experiments/sdl3/Cargo.toml','experiments/sdl3/Cargo.lock',
    'experiments/sdl3/packaging/build-and-package-macos.cjs',
    'experiments/sdl3/packaging/source-provenance.cjs',
    'experiments/sdl3/packaging/pilot-resources.cjs',
    'experiments/sdl3/vendor/sdl3-sys/build.rs']) {
    if (!names.has(required)) throw Error('Required source missing from manifest: '+required);
  }
  if (manifest.local_changes.some(n => typeof n !== 'string' || !names.has(n)) ||
      new Set(manifest.local_changes).size !== manifest.local_changes.length)
    throw Error('Invalid local changes');
  return {bytes, source: {
    base_commit: manifest.base_commit,
    source_manifest_sha256: digest('sha256', bytes),
    source_state: 'base commit plus listed local changes',
    verification: 'listed file hashes checked; base commit association requires independent verification',
    local_changes: manifest.local_changes
  }};
}
module.exports = {verifySourceManifest};
