const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const crypto = require('node:crypto');
const {verifySourceManifest} = require('./source-provenance.cjs');
const blob = b => crypto.createHash('sha1').update(Buffer.concat([Buffer.from('blob '+b.length+'\0'),b])).digest('hex');
const required = ['LICENSE','experiments/sdl3/Cargo.toml','experiments/sdl3/Cargo.lock',
  'experiments/sdl3/packaging/build-and-package-macos.cjs',
  'experiments/sdl3/packaging/source-provenance.cjs',
  'experiments/sdl3/packaging/pilot-resources.cjs',
  'experiments/sdl3/vendor/sdl3-sys/build.rs'];

test('source provenance verifies listed bytes without recording host paths', t => {
  const root = fs.realpathSync(fs.mkdtempSync(path.join(os.tmpdir(),'sdl-source-fixture-')));
  t.after(() => fs.rmSync(root,{recursive:true,force:true}));
  const entries = required.map(name => {
    const p = path.join(root,name), b = Buffer.from('fixture: '+name);
    fs.mkdirSync(path.dirname(p),{recursive:true});fs.writeFileSync(p,b);
    return {path:name,git_blob:blob(b)};
  });
  const manifest = {schema:1,base_commit:'1044299df39d3788d9f7bcaded7d84b0809259ac',files:entries,
    local_changes:['experiments/sdl3/packaging/pilot-resources.cjs']};
  const file = path.join(root,'SOURCE-MANIFEST.json');
  const save = () => fs.writeFileSync(file,JSON.stringify(manifest)+'\n');
  save();
  const verified = verifySourceManifest(root,file);
  assert.equal(verified.source.base_commit,manifest.base_commit);
  assert.match(verified.source.verification,/requires independent verification/);
  assert(!JSON.stringify(verified.source).includes(root));
  assert(!verified.bytes.toString().includes(root));
  manifest.host_path = root;save();
  assert.throws(() => verifySourceManifest(root,file),/Invalid source manifest/);
  delete manifest.host_path;manifest.files.push({path:'C:\\\\private\\\\source',git_blob:'0'.repeat(40)});save();
  assert.throws(() => verifySourceManifest(root,file),/Invalid source manifest entry/);
  manifest.files.pop();save();
  fs.writeFileSync(path.join(root,required[0]),'tampered');
  assert.throws(() => verifySourceManifest(root,file),/Source hash mismatch/);
  fs.writeFileSync(path.join(root,required[0]),'fixture: '+required[0]);
  manifest.files.push({path:'../escape',git_blob:'0'.repeat(40)});save();
  assert.throws(() => verifySourceManifest(root,file),/Invalid source manifest entry/);
  manifest.files.pop();manifest.files.pop();save();
  assert.throws(() => verifySourceManifest(root,file),/Required source missing/);
});
