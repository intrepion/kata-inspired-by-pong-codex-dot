# Mobile Pong comparison

## Architecture

The first iOS and Android experiment uses Tauri 2 native shells and the existing
`clients/web-wasm` frontend. Gameplay remains in the shared Rust `pong-core`,
compiled through `pong-wasm`; JavaScript only handles Canvas, input and timing.
This deliberately compares native WebViews rather than claiming a native GPU renderer.

Touch controls: hold the large up/down buttons; release/cancel clears the input.
Start/restart and pause/resume have separate buttons. Losing focus pauses gameplay.
The layout accommodates device safe areas and narrow/landscape viewports.

## Setup and commands

Build the WASM assets with `scripts/build-wasm.sh` (or its PowerShell equivalent).
In `clients/tauri`, use the official `@tauri-apps/cli` npm package:

```sh
npx @tauri-apps/cli@2 ios init
npx @tauri-apps/cli@2 ios build --debug --target aarch64-sim --no-sign --ci
npx @tauri-apps/cli@2 android init
npx @tauri-apps/cli@2 android dev
```

Generated projects under `clients/tauri/gen` are intentionally ignored, matching
Tauri's existing setup. Re-run init on each machine. iOS requires full Xcode and
an installed simulator runtime. Android requires Java, Android SDK, NDK and an
emulator with hardware virtualization support.

## Verification checkpoint

- Mobile Rust entry point added; desktop main delegates to the same run function.
- iOS: Xcode 27.2 and iOS 18.4 simulator devices are installed on the Mac.
- iOS follow-up (2026-11-11): **simulator build, installation, and process launch passed** on iPhone16/iOS18.4 using Xcode27.2. Tauri CLI's automatic archive/dev path reports a misleading missing-SDK error, but the generated scheme lists iOS18.4 simulators as compatible. Supported workaround: keep the Tauri CLI coordinator running (`tauri ios dev --open --no-watch`) alongside the localhost frontend server, then use direct generic-simulator `xcodebuild`. Without the coordinator the Rust prebuild fails with localhost connection refused. With it the full Rust + Xcode build succeeds. Installed/launched app stayed running and requested HTML, CSS, JS, and WASM from localhost with HTTP200. **Visual gameplay remains unverified**: CUA rejects Simulator by name and full app path. No additional SDK/runtime or security setting changes were needed.

### Verified iOS development build workaround

Run the server from repository root and leave it running:
```sh
python3 -m http.server 5188 --bind 127.0.0.1 --directory clients/web-wasm
```
In another terminal, leave the coordinator running:
```sh
cd clients/tauri
CARGO_TARGET_DIR=/private/tmp/pong-tauri-target CARGO_BUILD_JOBS=1 ./node_modules/.bin/tauri ios dev --open --no-watch
```
Then, from repository root:
```sh
CARGO_TARGET_DIR=/private/tmp/pong-tauri-target CARGO_BUILD_JOBS=1 xcodebuild -jobs 1 -project clients/tauri/gen/apple/pong-tauri.xcodeproj -scheme pong-tauri_iOS -configuration debug -sdk iphonesimulator -destination 'generic/platform=iOS Simulator' -derivedDataPath /private/tmp/pong-tauri-ios-validation CODE_SIGNING_ALLOWED=NO build
```
This is a development build using the localhost frontend, not a standalone release archive or App Store package. Xcode's script needs npm and cargo on PATH. Check free disk before building; this validation ended with about5.6GiB available.
- Shared checks: 9 Rust core tests and 8 JS/WASM tests pass. Desktop recheck was interrupted to conserve resources, not a pass.
- Android Windows SDK inspection, build, launch and gameplay: pending.
- No store submission, signing account setup or publication has been performed.

Do not treat available toolchains or successful compilation as gameplay QA.
