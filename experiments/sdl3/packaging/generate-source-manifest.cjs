// Inventory selected local source against local Git HEAD; this does not
// authenticate that HEAD is the intended remote commit.
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const SCOPE = Object.freeze(['LICENSE','crates/pong-core','crates/pong-scenario',
  'crates/pong-benchmark','experiments/low-level-common','experiments/sdl3',
  'scenarios','scripts/packaging']);
const safe = p => typeof p === 'string' && p.length > 0 && !path.posix.isAbsolute(p) &&
  !/[\\\\:\0]/.test(p) && !p.split('/').some(part => !part || part === '.' || part === '..');
const inScope = p => SCOPE.some(s => p === s || p.startsWith(s+'/'));
const blob = bytes => crypto.createHash('sha1').update(Buffer.concat([
  Buffer.from('blob '+bytes.length+'\0'),bytes
])).digest('hex');
function generateSourceManifest(sourceRoot, allowedChanges = []) {
  const root = fs.realpathSync(sourceRoot);
  const env = Object.fromEntries(Object.entries(process.env).filter(([k]) => !k.startsWith('GIT_')));
  env.GIT_NO_LAZY_FETCH = '1';
  env.GIT_TERMINAL_PROMPT = '0';
  const git = (...args) => cp.execFileSync('git', ['-C',root,...args], {env, maxBuffer:32*1024*1024});
  if (fs.realpathSync(git('rev-parse','--show-toplevel').toString().trim()) !== root)
    throw Error('Source root must be the Git worktree root');
  if (git('rev-parse','--show-object-format').toString().trim() !== 'sha1')
    throw Error('Only SHA-1 Git repositories are supported');
  const commit = git('rev-parse','--verify','HEAD^{commit}').toString().trim();
  if (!/^[0-9a-f]{40}$/.test(commit)) throw Error('Invalid local HEAD');
  const treeBytes = git('ls-tree','-r','-z','--full-tree',commit,'--',...SCOPE);
  const treeText = treeBytes.toString('utf8');
  if (!Buffer.from(treeText,'utf8').equals(treeBytes)) throw Error('Non-UTF-8 Git path');
  const tracked = new Map();
  for (const row of treeText.split('\0').filter(Boolean)) {
    const m = /^(100644|100755) blob ([0-9a-f]{40})\t([\s\S]+)$/.exec(row);
    if (!m || !safe(m[3]) || !inScope(m[3]) || tracked.has(m[3])) throw Error('Unsupported Git tree entry');
    tracked.set(m[3],m[2]);
  }
  if (!tracked.size) throw Error('No tracked source found');
  const allowed = new Set();
  for (const name of allowedChanges) {
    if (!safe(name) || !name.startsWith('experiments/sdl3/packaging/') || allowed.has(name))
      throw Error('Invalid allowed change');
    allowed.add(name);
  }
  const actual = new Set();
  function scopeAncestorsSafe(relative) {
    const parts = relative.split('/');
    let prefix = root;
    for (const part of parts.slice(0,-1)) {
      prefix = path.join(prefix,part);
      let stat;
      try { stat = fs.lstatSync(prefix); } catch (e) {
        if (e.code === 'ENOENT') return false;
        throw e;
      }
      if (stat.isSymbolicLink()) throw Error('Symlink source ancestor refused: '+relative);
      if (!stat.isDirectory()) throw Error('Non-directory source ancestor refused: '+relative);
    }
    return true;
  }
  function visit(relative) {
    const file = path.join(root,relative);
    let stat;
    try { stat = fs.lstatSync(file); } catch (e) {
      if (e.code === 'ENOENT') return;
      throw e;
    }
    if (stat.isSymbolicLink()) throw Error('Symlink source refused: '+relative);
    if (stat.isDirectory()) {
      // Cargo output is not a source input and may be large.
      if (path.posix.basename(relative) === 'target') return;
      for (const name of fs.readdirSync(file).sort()) visit(relative+'/'+name);
    } else if (stat.isFile()) {
      if (!safe(relative) || actual.has(relative)) throw Error('Invalid source path: '+relative);
      actual.add(relative);
    } else throw Error('Non-regular source refused: '+relative);
  }
  for (const scope of SCOPE) if (scopeAncestorsSafe(scope)) visit(scope);
  for (const name of tracked.keys()) if (!actual.has(name)) throw Error('Tracked source missing: '+name);
  const files = [], changed = [];
  for (const name of [...actual].sort()) {
    const before = tracked.get(name) ?? null;
    const current = blob(fs.readFileSync(path.join(root,name)));
    if (current !== before) {
      if (!allowed.has(name)) throw Error('Unexpected source change: '+name);
      changed.push(name);
      files.push({path:name,git_blob:current,base_git_blob:before});
    } else files.push({path:name,git_blob:current});
  }
  for (const name of allowed) if (!changed.includes(name)) throw Error('Allowed change is absent or unchanged: '+name);
  if (git('rev-parse','--verify','HEAD^{commit}').toString().trim() !== commit)
    throw Error('HEAD changed during source inventory');
  return {schema:1,base_commit:commit,files,local_changes:changed};
}
function verifyGeneratedManifest(sourceRoot, bytes) {
  const supplied = JSON.parse(bytes.toString('utf8'));
  if (!Array.isArray(supplied?.local_changes)) throw Error('Invalid source manifest');
  const regenerated = Buffer.from(JSON.stringify(generateSourceManifest(sourceRoot,supplied.local_changes),null,2)+'\n');
  if (!bytes.equals(regenerated)) throw Error('Source manifest differs from local Git inventory');
}
if (require.main === module) {
  const args = process.argv.slice(2), allowed = [];
  while (args.length) {
    if (args.shift() !== '--allow-change' || !args.length) throw Error('Usage: node generate-source-manifest.cjs [--allow-change RELATIVE_PATH]...');
    allowed.push(args.shift());
  }
  const root = path.resolve(__dirname,'../../..');
  process.stdout.write(JSON.stringify(generateSourceManifest(root,allowed),null,2)+'\n');
}
module.exports = {generateSourceManifest, verifyGeneratedManifest, SCOPE};
