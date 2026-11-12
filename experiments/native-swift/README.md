# Native Swift Pong

Normal gameplay uses SwiftUI and independent Swift physics. No Rust engine is embedded.

## Debug deterministic replay

Generate the Xcode project with `xcodegen generate` and build the Debug app.
Launch with `--scripted-test` (Xcode scheme launch arguments or `xcrun simctl launch booted com.intrepion.pong.native.swift --scripted-test`).

The DEBUG-only adapter loads the bundled shared `smoke-v1.txt` manifest, advances two fixed 120Hz ticks per timer callback, and ignores live controls and focus changes during replay. Normal launches retain touch movement, pause, resume, and restart. Release builds exclude replay/export logic.

Each replay creates a new `Documents/replay-UUID` inside this app's sandbox, never overwriting previous evidence:

- `telemetry.jsonl`: 13 state reports including tick 0 and finished tick 1440.
- Six `tick-NNNN.png` files at manifest checkpoints.
- `complete.txt` appears only after the final report and render export succeeded.

PNG files are app-owned SwiftUI `ImageRenderer` exports using the same `PongCourt` as normal gameplay, at 900×560 including a score/state header. They are **not screenshots of the displayed simulator/window compositor**, nor evidence of keyboard/touch interaction. The live replay finishes in-place with a completion label; it does not terminate the app.

Locate exports with `xcrun simctl get_app_container booted com.intrepion.pong.native.swift data`, then inspect that directory's `Documents` folder.

## Tests

- `swift test`: independent physics plus shared-manifest parser/replay tests.
- `xcodebuild -project PongNativeSwift.xcodeproj -scheme PongNativeSwift -destination 'platform=iOS Simulator,name=iPhone 16 Pro' CODE_SIGNING_ALLOWED=NO test`: debug replay completion/disabled controls plus normal start/pause/resume/restart UI regression.

The Swift scenario follows the same event/input fixture, but cross-language floating-point/physics parity must not be assumed from scenario completion alone.
