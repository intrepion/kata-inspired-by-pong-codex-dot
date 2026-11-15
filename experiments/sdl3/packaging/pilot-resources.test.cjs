const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const {writePilotResources, verifyPilotResources} = require('./pilot-resources.cjs');
const hash = b => crypto.createHash('sha256').update(b).digest('hex');
const resources = 'Pong SDL3.app/Contents/Resources';
const files = ['THIRD-PARTY-NOTICES.txt','NOTICE-INDEX.json','BUILD-PROVENANCE.json','SOURCE-MANIFEST.json'];

test('app and archive retain exact notices, index and provenance', t => {
  const out = fs.mkdtempSync(path.join(os.tmpdir(), 'pong-sdl3-resources-'));
  t.after(() => fs.rmSync(out, {recursive:true, force:true}));
  fs.mkdirSync(path.join(out, resources), {recursive:true});
  const notice = 'Sun notice\nKhronos copyright and permission\n';
  const notices = {text:notice, index:{bundle_sha256:hash(Buffer.from(notice)), missing_notice_files:[]}};
  const source = Buffer.from('{"base_commit":"1044299"}\n');
  const provenance = {schema:3,build_binary_sha256:'binary-hash',target:'aarch64-apple-darwin',source:{base_commit:'1044299',source_manifest_sha256:hash(source)}};
  const expected = writePilotResources(out, notices, provenance, 'binary-hash', source);
  const archive = path.join(out, 'pilot.tar.gz');
  const makeArchive = () => cp.execFileSync('tar', ['-czf',archive,'-C',out,'Pong SDL3.app',...files]);
  makeArchive();
  assert.doesNotThrow(() => verifyPilotResources(out, archive, expected));
  for (const name of files) {
    const inside = cp.execFileSync('tar',['-xOf',archive,resources+'/'+name]);
    const outside = cp.execFileSync('tar',['-xOf',archive,name]);
    assert(inside.equals(outside));
    assert.equal(hash(inside),hash(expected[name]));
  }
  assert.equal(JSON.parse(expected['NOTICE-INDEX.json']).bundle_sha256, hash(expected['THIRD-PARTY-NOTICES.txt']));
  assert.equal(JSON.parse(expected['BUILD-PROVENANCE.json']).build_binary_sha256, 'binary-hash');
  assert.equal(JSON.parse(expected['BUILD-PROVENANCE.json']).source.source_manifest_sha256,hash(expected['SOURCE-MANIFEST.json']));
  assert(!expected['BUILD-PROVENANCE.json'].toString().includes(out));
  assert(!expected['SOURCE-MANIFEST.json'].toString().includes(out));
  for (const name of files) {
    fs.writeFileSync(path.join(out, name), 'tampered');
    assert.throws(() => verifyPilotResources(out, archive, expected), /packaged resource mismatch/);
    fs.writeFileSync(path.join(out, name), expected[name]);
    fs.writeFileSync(path.join(out, resources, name), 'tampered');
    assert.throws(() => verifyPilotResources(out, archive, expected), /packaged resource mismatch/);
    makeArchive();
    fs.writeFileSync(path.join(out, resources, name), expected[name]);
    assert.throws(() => verifyPilotResources(out, archive, expected), /archived resource mismatch/);
    makeArchive();
  }
});

test('inconsistent notice index and binary provenance fail closed', t => {
  const out = fs.mkdtempSync(path.join(os.tmpdir(), 'pong-sdl3-resource-invalid-'));
  t.after(() => fs.rmSync(out, {recursive:true, force:true}));
  fs.mkdirSync(path.join(out, resources), {recursive:true});
  const source = Buffer.from('{}');
  assert.throws(() => writePilotResources(out, {text:'a',index:{bundle_sha256:'wrong'}}, {build_binary_sha256:'x'}, 'x', source), /notice index hash mismatch/);
  assert.throws(() => writePilotResources(out, {text:'a',index:{bundle_sha256:hash(Buffer.from('a'))}}, {build_binary_sha256:'wrong'}, 'x', source), /provenance binary hash mismatch/);
  assert.throws(() => writePilotResources(out, {text:'a',index:{bundle_sha256:hash(Buffer.from('a'))}}, {build_binary_sha256:'x',source:{source_manifest_sha256:'wrong'}}, 'x', source), /source manifest hash mismatch/);
  assert.equal(fs.existsSync(path.join(out, files[0])), false);
});
