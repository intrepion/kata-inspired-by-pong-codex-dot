// Detect a changed app copy or archive after the source build.
const fs = require('node:fs');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');
module.exports = function verifyPackagedBinary(expected, packed, archive) {
  if (hash(fs.readFileSync(packed)) !== expected) throw Error('packaged binary differs from verified build');
  const archived = cp.execFileSync('tar', ['-xOf', archive, 'Pong SDL3.app/Contents/MacOS/pong-sdl3'], {maxBuffer:16*1024*1024});
  if (hash(archived) !== expected) throw Error('archived binary differs from verified build');
};
