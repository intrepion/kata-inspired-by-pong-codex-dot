// Keep the app bundle and archive-level review files byte-identical.
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const hash = b => crypto.createHash('sha256').update(b).digest('hex');
const resources = 'Pong SDL3.app/Contents/Resources';

function writePilotResources(out, notices, provenance, binaryHash, sourceManifest) {
  const noticeBytes = Buffer.from(notices.text, 'utf8');
  if (notices.index.bundle_sha256 !== hash(noticeBytes)) throw Error('notice index hash mismatch');
  if (provenance.build_binary_sha256 !== binaryHash) throw Error('provenance binary hash mismatch');
  if (!Buffer.isBuffer(sourceManifest) || provenance.source?.source_manifest_sha256 !== hash(sourceManifest))
    throw Error('provenance source manifest hash mismatch');
  const files = {
    'THIRD-PARTY-NOTICES.txt': noticeBytes,
    'NOTICE-INDEX.json': Buffer.from(JSON.stringify(notices.index, null, 2)+'\n'),
    'BUILD-PROVENANCE.json': Buffer.from(JSON.stringify(provenance, null, 2)+'\n'),
    'SOURCE-MANIFEST.json': sourceManifest
  };
  for (const [name, bytes] of Object.entries(files)) {
    fs.writeFileSync(path.join(out, name), bytes);
    fs.writeFileSync(path.join(out, resources, name), bytes);
  }
  return files;
}

function verifyPilotResources(out, archive, expected) {
  for (const [name, bytes] of Object.entries(expected)) {
    for (const relative of [name, resources+'/'+name]) {
      if (!fs.readFileSync(path.join(out, relative)).equals(bytes))
        throw Error('packaged resource mismatch: '+relative);
      const archived = cp.execFileSync('tar', ['-xOf', archive, relative], {maxBuffer:64*1024*1024});
      if (!archived.equals(bytes)) throw Error('archived resource mismatch: '+relative);
    }
  }
}
module.exports = {writePilotResources, verifyPilotResources};
