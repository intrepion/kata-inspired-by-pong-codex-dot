// Local unsigned distribution pilot. No system install or signing changes.
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const [binaryArg, outputArg] = process.argv.slice(2);
if (!binaryArg || !outputArg) throw Error('Usage: node package-macos.cjs RELEASE_BINARY NEW_OUTPUT_DIRECTORY');
if (process.platform !== 'darwin') throw Error('macOS packaging pilot must run on macOS');
const binary = path.resolve(binaryArg), out = path.resolve(outputArg);
if (!fs.statSync(binary).isFile()) throw Error('binary must be a regular file');
const format = cp.execFileSync('file', ['-b', binary], {encoding:'utf8'});
if (!format.includes('Mach-O 64-bit executable arm64')) throw Error('pilot requires a thin arm64 Mach-O executable');
fs.mkdirSync(out); // Refuse to overwrite any existing output.
const root = path.resolve(__dirname, '../../..');
const contents = path.join(out, 'Pong Softbuffer.app/Contents');
fs.mkdirSync(path.join(contents, 'MacOS'), {recursive: true});
fs.mkdirSync(path.join(contents, 'Resources'));
fs.copyFileSync(binary, path.join(contents, 'MacOS/pong-softbuffer'));
fs.chmodSync(path.join(contents, 'MacOS/pong-softbuffer'), 0o755);
fs.copyFileSync(path.join(root, 'LICENSE'), path.join(contents, 'Resources/LICENSE'));
fs.writeFileSync(path.join(contents, 'Info.plist'), `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleName</key><string>Pong Softbuffer</string>
<key>CFBundleIdentifier</key><string>com.intrepion.kata.pong.softbuffer.pilot</string>
<key>CFBundleExecutable</key><string>pong-softbuffer</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleVersion</key><string>0.0.1</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>\n`);
fs.writeFileSync(path.join(out, 'PILOT-NOTICE.txt'), 'Local packaging experiment only. Not signed for distribution or notarized. Third-party license inventory, downloaded/quarantined Gatekeeper behavior, clean-machine installation and upgrades have NOT been qualified. Do not disable Gatekeeper to run it.\n');
cp.execFileSync('plutil', ['-lint', path.join(contents, 'Info.plist')], {stdio:'inherit'});
const archive = path.join(out, 'pong-softbuffer-macos-arm64-pilot.tar.gz');
cp.execFileSync('tar', ['-czf', archive, '-C', out, 'Pong Softbuffer.app', 'PILOT-NOTICE.txt']);
const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const manifest = {schema:1, platform: process.platform, architecture:process.arch, binary_sha256:hash(binary), archive_sha256:hash(archive), archive_bytes:fs.statSync(archive).size};
fs.writeFileSync(path.join(out,'manifest.json'),JSON.stringify(manifest,null,2)+'\n');
console.log(JSON.stringify(manifest));
