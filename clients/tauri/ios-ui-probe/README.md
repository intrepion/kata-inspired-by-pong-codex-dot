# Tauri iOS input probe

This separate XCTest host launches the installed `com.intrepion.katapongcomparison` app and tests the uninstrumented webview's Start, Pause and Resume controls via accessibility. It does not build or install the Tauri app. Use a previously built simulator development shell configured for `http://127.0.0.1:5188`, and build and copy `pong_wasm.wasm` into `clients/web-wasm` as described in `clients/web-wasm/SCRIPTED-DEMO.md`. Start the loopback server from the repository root:

```sh
python3 -m http.server 5188 --bind 127.0.0.1 --directory clients/web-wasm
```

Boot an iOS simulator with that Tauri app already installed. With Xcode and XcodeGen installed, generate the separate test project and run (replace the simulator ID and use a new result bundle path):

```sh
cd clients/tauri/ios-ui-probe
xcodegen generate
xcodebuild test -project TauriInputProbe.xcodeproj -scheme ProbeHost \
  -destination 'platform=iOS Simulator,id=SIMULATOR_ID' \
  -derivedDataPath /tmp/pong-tauri-xcui-derived \
  -resultBundlePath /tmp/pong-tauri-input-xcui.xcresult \
  -only-testing:TauriInputTests/TauriInputTests/testStartPauseResume CODE_SIGNING_ALLOWED=NO
```

The UI test waits for Ready, taps Start, Pause and Resume, and checks Playing, Paused and Playing. It generates simulator input; it does not establish physical touch, simultaneous touches, a full match, release packaging or accessibility quality. The test adds a separate host and test runner to the simulator; it does not uninstall apps or erase simulator data.

For the unattended match-over/restart check, replace the `-only-testing` value with `TauriInputTests/TauriInputTests/testMatchOverAndRestart` and use a fresh result-bundle path. The test checks the Match over and subsequent Playing states, not the final or reset score.
