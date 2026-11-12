// Local unsigned portable pilot. Never installs, changes PATH, or deletes files.
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
function packageWindows(binaryArg, outputArg) {
  if (process.platform !== 'win32') throw Error('Windows pilot requires Windows');
  const binary = path.resolve(binaryArg), out = path.resolve(outputArg);
  const bytes = fs.readFileSync(binary);
  if (bytes.length < 64 || bytes.readUInt16LE(0) !== 0x5a4d) throw Error('Expected PE executable');
  const pe = bytes.readUInt32LE(0x3c);
  if (pe + 6 > bytes.length || bytes.readUInt32LE(pe) !== 0x4550 || bytes.readUInt16LE(pe + 4) !== 0x8664) throw Error('Expected x64 PE executable');
  fs.mkdirSync(out); // Deliberately refuse existing output.
  const stage = path.join(out, 'Pong Macroquad');
  fs.mkdirSync(stage);
  const root = path.resolve(__dirname, '../../..');
  fs.copyFileSync(binary, path.join(stage, 'pong-macroquad.exe'));
  fs.copyFileSync(path.join(root, 'LICENSE'), path.join(stage, 'LICENSE'));
  fs.writeFileSync(path.join(stage, 'PILOT-NOTICE.txt'), 'Local unsigned portable packaging pilot. No installer, PATH edits or registry changes. Third-party notices, clean-machine dependencies, downloaded-file SmartScreen behavior and upgrades are NOT qualified. Do not bypass security warnings.\n');
  const archive = path.join(out, 'pong-macroquad-windows-x64-pilot.zip');
  cp.execFileSync('powershell.exe', ['-NoProfile', '-NonInteractive', '-Command', 'Compress-Archive -LiteralPath $env:PONG_PACKAGE_STAGE -DestinationPath $env:PONG_PACKAGE_ARCHIVE -ErrorAction Stop'], {env:{...process.env,PONG_PACKAGE_STAGE:stage,PONG_PACKAGE_ARCHIVE:archive}, windowsHide:true});
  const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
  const manifest = {schema:1, platform:'win32', architecture:'x64', binary_sha256:hash(binary), archive_sha256:hash(archive), archive_bytes:fs.statSync(archive).size};
  fs.writeFileSync(path.join(out, 'manifest.json'), JSON.stringify(manifest,null,2)+'\n');
  return manifest;
}
if (require.main === module) {
  const [binary, out] = process.argv.slice(2);
  if (!binary || !out) throw Error('Usage: node package-windows.cjs RELEASE_EXE NEW_OUTPUT_DIRECTORY');
  console.log(JSON.stringify(packageWindows(binary,out)));
}
module.exports = {packageWindows};
