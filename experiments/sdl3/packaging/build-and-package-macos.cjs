// Local unsigned pilot: build the package input from the checked-in SDL source.
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const root = path.resolve(__dirname, '../../..');
const manifest = path.join(root, 'experiments/sdl3/Cargo.toml');
const vendored = path.join(root, 'experiments/sdl3/vendor/sdl3-sys/Cargo.toml');
const outputArg = process.argv[2];
if (process.argv.length !== 3 || !outputArg) throw Error('Usage: node build-and-package-macos.cjs NEW_OUTPUT_DIRECTORY');
if (process.platform !== 'darwin' || process.arch !== 'arm64') throw Error('requires an arm64 macOS host');
const out = path.resolve(outputArg);
if (fs.existsSync(out)) throw Error('output directory must not already exist');
if (Object.keys(process.env).some(k => k.startsWith('CMAKE_TOOLCHAIN_FILE'))) throw Error('external CMake toolchain overrides are not supported');
const metadata = JSON.parse(cp.execFileSync('cargo', ['metadata', '--format-version=1', '--offline', '--locked', '--manifest-path', manifest], {encoding:'utf8', maxBuffer:16*1024*1024}));
const tree = cp.execFileSync('cargo', ['tree','--offline','--locked','--manifest-path',manifest,'--target','aarch64-apple-darwin','--edges','normal,build','--prefix','none','--format','{p}'], {encoding:'utf8',maxBuffer:16*1024*1024});
const notices = JSON.parse(cp.execFileSync(process.execPath, [path.join(__dirname, 'create-macos-notices.mjs')], {input:JSON.stringify({metadata,tree}), encoding:'utf8', maxBuffer:64*1024*1024}));
const selected = metadata.packages.filter(p => p.name === 'sdl3-sys');
if (selected.length !== 1 || fs.realpathSync(selected[0].manifest_path) !== fs.realpathSync(vendored)) throw Error('expected the scoped vendored sdl3-sys');
const target = fs.mkdtempSync(path.join(os.tmpdir(), 'pong-sdl3-macos-build-'));
const env = {...process.env, CARGO_TARGET_DIR:target};
cp.execFileSync('cargo', ['build', '--release', '--offline', '--locked', '--target', 'aarch64-apple-darwin', '--manifest-path', manifest], {env, stdio:'inherit'});
const release = path.join(target, 'aarch64-apple-darwin/release');
const build = path.join(release, 'build');
const caches = fs.readdirSync(build).filter(n => n.startsWith('sdl3-sys-')).map(n => path.join(build,n,'out/build/CMakeCache.txt')).filter(fs.existsSync);
if (caches.length !== 1) throw Error('expected one SDL CMake cache in the fresh target');
const cache = fs.readFileSync(caches[0], 'utf8');
for (const [key,value] of [['SDL_HIDAPI_LIBUSB','OFF'],['SDL_HIDAPI','ON'],['SDL_HIDAPI_JOYSTICK','ON'],['SDL_JOYSTICK','ON']]) {
  if (!cache.split(/\r?\n/).includes(key+':BOOL='+value)) throw Error('unexpected SDL configuration: '+key);
}
const binary = path.join(release, 'pong-sdl3');
const symbols = cp.execFileSync('nm', ['-m', binary], {encoding:'utf8', maxBuffer:16*1024*1024});
const strings = cp.execFileSync('strings', [binary], {encoding:'utf8', maxBuffer:16*1024*1024});
if (!symbols.includes('PLATFORM_hid_') || /LIBUSB_hid_|SDL_InitLibUSB/.test(symbols) || /libusb-1\.0\.dylib|libusb_init/.test(strings)) throw Error('unexpected HID backend in built binary');
const hash = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
const binaryHash = hash(binary);
// This function has no caller-supplied binary argument: it packages only the
// build from the fresh target created above.
fs.mkdirSync(out);
const contents = path.join(out, 'Pong SDL3.app/Contents');
fs.mkdirSync(path.join(contents, 'MacOS'), {recursive:true});
fs.mkdirSync(path.join(contents, 'Resources'));
const packed = path.join(contents, 'MacOS/pong-sdl3');
fs.copyFileSync(binary, packed);
fs.chmodSync(packed, 0o755);
fs.copyFileSync(path.join(root, 'LICENSE'), path.join(contents, 'Resources/LICENSE'));
fs.writeFileSync(path.join(contents, 'Info.plist'), `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleName</key><string>Pong SDL3</string>
<key>CFBundleIdentifier</key><string>com.intrepion.kata.pong.sdl3.pilot</string>
<key>CFBundleExecutable</key><string>pong-sdl3</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleVersion</key><string>0.0.1</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>\n`);
fs.writeFileSync(path.join(out, 'PILOT-NOTICE.txt'), 'Local packaging experiment only. Not signed for distribution or notarized. Third-party license inventory, downloaded/quarantined Gatekeeper behavior, clean-machine installation and upgrades have NOT been qualified. Do not disable Gatekeeper to run it.\n');
fs.writeFileSync(path.join(out, 'THIRD-PARTY-NOTICES.txt'), notices.text);
fs.writeFileSync(path.join(out, 'NOTICE-INDEX.json'), JSON.stringify(notices.index,null,2)+'\n');
cp.execFileSync('plutil', ['-lint', path.join(contents, 'Info.plist')], {stdio:'inherit'});
const archive = path.join(out, 'pong-sdl3-macos-arm64-pilot.tar.gz');
cp.execFileSync('tar', ['-czf', archive, '-C', out, 'Pong SDL3.app', 'PILOT-NOTICE.txt', 'THIRD-PARTY-NOTICES.txt', 'NOTICE-INDEX.json']);
require('./verify-packaged-binary.cjs')(binaryHash, packed, archive);
const packageManifest = {schema:1, platform:process.platform, architecture:process.arch, binary_sha256:binaryHash, archive_sha256:hash(archive), archive_bytes:fs.statSync(archive).size, notices_sha256:hash(path.join(out,'THIRD-PARTY-NOTICES.txt')), missing_notice_files:notices.index.missing_notice_files};
fs.writeFileSync(path.join(out,'manifest.json'), JSON.stringify(packageManifest,null,2)+'\n');
fs.writeFileSync(path.join(out,'BUILD-PROVENANCE.json'), JSON.stringify({schema:1, target:'aarch64-apple-darwin', sdl_hidapi_libusb:false, native_hidapi:true, joystick:true, binary_sha256:binaryHash, cargo_lock_sha256:hash(path.join(root,'experiments/sdl3/Cargo.lock')), sdl_sys_build_sha256:hash(path.join(root,'experiments/sdl3/vendor/sdl3-sys/build.rs')), build_target:target},null,2)+'\n');
console.log(JSON.stringify(packageManifest));
console.log('Verified local build target: '+target);
