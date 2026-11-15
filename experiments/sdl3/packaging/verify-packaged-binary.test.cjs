const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const verify = require('./verify-packaged-binary.cjs');
const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');

test('direct packager and build wrapper reject arbitrary binary arguments', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(),'pong-sdl3-reject-test-'));
  const out = path.join(root,'output');
  const binary = path.join(root,'arbitrary');
  fs.writeFileSync(binary,'not a verified build');
  for (const name of ['package-macos.cjs','build-and-package-macos.cjs']) {
    const result = cp.spawnSync(process.execPath, [path.join(__dirname,name), binary, out], {encoding:'utf8'});
    assert.notEqual(result.status, 0, name);
    assert.equal(fs.existsSync(out), false, name);
  }
});

test('integrity check rejects tampered app and archived executables', () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(),'pong-sdl3-integrity-test-'));
  const packed = path.join(root,'Pong SDL3.app/Contents/MacOS/pong-sdl3');
  fs.mkdirSync(path.dirname(packed), {recursive:true});
  fs.writeFileSync(packed,'verified build');
  const expected = hash(packed);
  const archive = path.join(root,'pilot.tar.gz');
  const createArchive = () => cp.execFileSync('tar',['-czf',archive,'-C',root,'Pong SDL3.app']);
  createArchive();
  assert.doesNotThrow(() => verify(expected, packed, archive));
  fs.writeFileSync(packed,'tampered app');
  assert.throws(() => verify(expected, packed, archive), /packaged binary differs/);
  createArchive();
  fs.writeFileSync(packed,'verified build');
  assert.throws(() => verify(expected, packed, archive), /archived binary differs/);
});
