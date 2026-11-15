const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const cp = require('node:child_process');
const {generateSourceManifest, verifyGeneratedManifest} = require('./generate-source-manifest.cjs');
const {verifySourceManifest} = require('./source-provenance.cjs');

function fixture(t) {
  const root = fs.realpathSync(fs.mkdtempSync(path.join(os.tmpdir(),'sdl-manifest-git-')));
  t.after(() => fs.rmSync(root,{recursive:true,force:true}));
  const git = (...args) => cp.execFileSync('git',['-C',root,...args],{stdio:'pipe'});
  const put = (name,text) => {const f=path.join(root,name);fs.mkdirSync(path.dirname(f),{recursive:true});fs.writeFileSync(f,text);};
  git('init','-q');
  put('LICENSE','license\n');
  put('experiments/sdl3/Cargo.toml','baseline\n');
  put('experiments/sdl3/Cargo.lock','lock\n');
  put('experiments/sdl3/.gitignore','*.local\n');
  put('experiments/sdl3/packaging/build-and-package-macos.cjs','packager\n');
  put('experiments/sdl3/packaging/source-provenance.cjs','source helper\n');
  put('experiments/sdl3/packaging/pilot-resources.cjs','resource helper\n');
  put('experiments/sdl3/packaging/unchanged.cjs','unchanged\n');
  put('experiments/sdl3/vendor/sdl3-sys/build.rs','build script\n');
  put('crates/pong-core/src/lib.rs','core\n');
  git('add','--all');
  git('-c','user.name=Fixture','-c','user.email=fixture@example.invalid','-c','commit.gpgsign=false','commit','-qm','fixture');
  return {root,git,put};
}
test('clean local Git inventory is deterministic and ignores target output', t => {
  const {root,git,put}=fixture(t);
  put('experiments/sdl3/target/release/cache','not source');
  const first=generateSourceManifest(root),second=generateSourceManifest(root);
  assert.equal(JSON.stringify(first),JSON.stringify(second));
  assert.equal(first.base_commit,git('rev-parse','HEAD').toString().trim());
  assert.deepEqual(first.local_changes,[]);
  assert(!first.files.some(x=>x.path.includes('/target/')));
  assert(!JSON.stringify(first).includes(root));
  const manifestFile=path.join(root,'SOURCE-MANIFEST.json');
  fs.writeFileSync(manifestFile,JSON.stringify(first,null,2)+'\n');
  const checked=verifySourceManifest(root,manifestFile);
  assert.equal(checked.source.base_commit,first.base_commit);
  assert.doesNotThrow(()=>verifyGeneratedManifest(root,checked.bytes));
});
test('changed and extra source must be named explicitly, including ignored files', t => {
  const {root,put}=fixture(t);
  const changed='experiments/sdl3/packaging/build-and-package-macos.cjs',extra='experiments/sdl3/packaging/patch.local';
  put(changed,'changed\n');put(extra,'new\n');
  assert.throws(()=>generateSourceManifest(root),/Unexpected source change/);
  assert.throws(()=>generateSourceManifest(root,[changed]),/Unexpected source change/);
  const a=generateSourceManifest(root,[extra,changed]),b=generateSourceManifest(root,[changed,extra]);
  assert.deepEqual(a,b);
  assert.deepEqual(a.local_changes,[changed,extra]);
  assert.match(a.files.find(x=>x.path===changed).base_git_blob,/^[0-9a-f]{40}$/);
  assert.equal(a.files.find(x=>x.path===extra).base_git_blob,null);
  assert.throws(()=>generateSourceManifest(root,[changed,extra,'experiments/sdl3/packaging/unchanged.cjs']),/absent or unchanged/);
  assert.throws(()=>generateSourceManifest(root,[changed,extra,'experiments/sdl3/Cargo.toml']),/Invalid allowed change/);
});
test('missing tracked files, unsafe paths and symlinks fail closed', t => {
  const {root,put}=fixture(t);
  assert.throws(()=>generateSourceManifest(root,['../escape']),/Invalid allowed change/);
  assert.throws(()=>generateSourceManifest(root,['experiments/sdl3/packaging/x','experiments/sdl3/packaging/x']),/Invalid allowed change/);
  fs.unlinkSync(path.join(root,'LICENSE'));
  assert.throws(()=>generateSourceManifest(root),/Tracked source missing/);
  put('LICENSE','license\n');
  fs.symlinkSync(path.join(root,'LICENSE'),path.join(root,'experiments/sdl3/packaging/link'));
  assert.throws(()=>generateSourceManifest(root,['experiments/sdl3/packaging/link']),/Symlink source refused/);
});
test('packager rejects stale, incomplete or falsely based manifests', t => {
  const {root,put}=fixture(t);
  const manifest=generateSourceManifest(root);
  const bytes=Buffer.from(JSON.stringify(manifest,null,2)+'\n');
  assert.doesNotThrow(()=>verifyGeneratedManifest(root,bytes));
  const forged={...manifest,base_commit:'0'.repeat(40)};
  assert.throws(()=>verifyGeneratedManifest(root,Buffer.from(JSON.stringify(forged,null,2)+'\n')),/differs from local Git inventory/);
  const incomplete={...manifest,files:manifest.files.filter(x=>x.path!=='LICENSE')};
  assert.throws(()=>verifyGeneratedManifest(root,Buffer.from(JSON.stringify(incomplete,null,2)+'\n')),/differs from local Git inventory/);
  put('experiments/sdl3/new-source.c','unexpected');
  assert.throws(()=>verifyGeneratedManifest(root,bytes),/Unexpected source change/);
});
test('generator refuses a symlink above a scoped source root', t => {
  const {root}=fixture(t);
  fs.renameSync(path.join(root,'experiments'),path.join(root,'real-experiments'));
  fs.symlinkSync(path.join(root,'real-experiments'),path.join(root,'experiments'));
  assert.throws(()=>generateSourceManifest(root),/Symlink source ancestor refused/);
});
