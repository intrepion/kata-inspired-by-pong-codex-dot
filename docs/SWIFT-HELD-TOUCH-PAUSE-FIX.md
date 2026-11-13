# Swift held touch across pause

Source review identified that the SwiftUI court's target persisted across Pause/Resume. A source-derived macOS model probe using the existing `Game` and the original target-to-axis expression reproduced the consequence: a retained target of 450 moved the player from 250 to 265.51 on the first step after pause/resume. This is a model reproduction, not proof that iOS delivered overlapping touches.

A `TouchInput` state helper now suppresses an active drag when play pauses, restarts or the app becomes inactive. Continued callbacks from that drag cannot restore the target; gesture end or SwiftUI gesture-state cancellation clears it, and a fresh drag can move. A drag begun while not playing is likewise suppressed until it ends. The timer consumes the helper's axis. No shared Rust physics or Android code changed.

On macOS ARM64, `swift test` passed 14 tests, including three new cases for pause/resume with continued callbacks, release/cancellation, fresh gesture, and pause without an active gesture. Restricted SwiftPM attempts could not initialize its own sandbox; the successful run used approved host execution with an isolated scratch directory. An isolated Xcode project and copied derived-data cache produced a successful generic iOS Simulator debug build. Tested SwiftUI app blob `05048d2b2f8e3ae6d4edd472fe4de4be8a0e2f8c`, touch model blob `167604846a28f6022f6322a78fef572f78c8a81d`, based on checkpoint `18869473f3b703f7ceab6bce898936a135ba7df7`.

An iPhone 16 Pro simulator UI-test attempt compiled the app, but SpringBoard reported that it denied the app launch (`FBSOpenApplicationServiceErrorDomain`, `RequestDenied`); no UI test completed. The stalled test was stopped, and the simulator was already shut down. The build-only check then succeeded. Overlapping real simulator gestures, gesture cancellation on a device, physical input and release qualification remain open. Logs and isolated build artifacts remain under `/tmp/pong-swift-held-pause-*`; earlier evidence was preserved.

## Simulator launch follow-up

The earlier failed test launch is retained above. The simulator was booted normally and `simctl bootstatus -b` reported completion; the already-installed test app then launched normally. Its executable SHA-256 `e96fae13cac176fdcf4bb703afb5a18445b3db51a373314c22314118d054a10c` matched the isolated build. The original denial's cause was not established.

On the booted iPhone 16 Pro simulator, the existing `testStartPauseResumeRestart` passed (21.193 seconds), and `testTouchReleaseBackgroundAndFullMatch` passed (53.878 seconds), exercising a drag and release, background/resume, normal game-over and restart. These are simulator UI interactions; they do not hold a court touch while a second touch pauses and resumes. That overlapping-touch scenario and physical-device behavior remain unverified. The simulator was shut down without erasing its data. Result bundles and logs remain at `/tmp/pong-swift-held-pause-ui-retry.xcresult`, `/tmp/pong-swift-held-pause-touch-retry.xcresult` and matching `.log` files.

## Paused-drag simulator test

A new `testPausedDragRequiresFreshGestureAfterResume` ran on the iPhone 16 Pro simulator against the same Swift fix. It paused the game, dragged and released on the court, confirmed the paddle stayed fixed, resumed and confirmed it remained fixed, then confirmed a fresh drag moved the paddle. The test passed in 36.170 seconds; result bundle `/tmp/pong-swift-paused-drag-ui.xcresult` and log `/tmp/pong-swift-paused-drag-ui.log` are retained. The simulator was shut down without erasing data. This is a sequential simulator gesture test; a court touch held while another touch activates Pause/Resume and physical-device input remain unverified.

## Release configuration check

The isolated project with the same SwiftUI and touch-model blobs above built successfully for generic iOS Simulator in Release configuration, using a separate copy-on-write derived-data directory. The universal simulator executable SHA-256 is `059d0e26765a43ac973c6e878edb42aac727778542f9dada19643380de07afd7`. Separately, `swift test -c release` passed all 14 package tests on macOS ARM64, including the three touch-state regressions. Build and test logs remain under `/tmp/pong-swift-held-release-*`. This is optimized build/model evidence; the Release app was not launched, and physical-device, overlapping-touch, signing and distribution qualification remain open.

## Release simulator UI smoke

A subsequent iPhone 16 Pro simulator run used `xcodebuild test -configuration Release` and passed the existing `testStartPauseResumeRestart` in 19.481 seconds. The installed Release executable matched the test build SHA-256 `7206bf7faa777c5d15969293794aef57e05f7f6d744442e7b7e4643ef64ee647` (the destination-specific test build differs from the earlier generic universal binary). Result bundle and log are `/tmp/pong-swift-release-ui-smoke.xcresult` and `/tmp/pong-swift-release-ui-smoke.log`. Afterward, the earlier debug app was reinstalled and its executable hash `e96fae13cac176fdcf4bb703afb5a18445b3db51a373314c22314118d054a10c` verified; the simulator was shut down without erasing data. This is a Release simulator UI smoke, not overlapping-touch, physical-device, signing or distribution qualification.

## Release simulator bundle metadata

A separate read-only inspection of the Release simulator app used in the UI smoke above confirmed that its executable still has SHA-256 `7206bf7faa777c5d15969293794aef57e05f7f6d744442e7b7e4643ef64ee647` and is a thin arm64 Mach-O. Its Info.plist identifies the iPhone Simulator platform, minimum iOS 17.0, and iPhone/iPad device families; it contains neither `CFBundleVersion` nor `CFBundleShortVersionString`. `codesign -dv` reports an ad-hoc, linker-signed bundle with no team identifier. These are properties of that historical simulator test artifact, not a device archive or a release candidate. Version metadata, device build and signing, icons, distribution and physical-device qualification remain open. No app or simulator data was modified by this inspection.

## Explicit simulator version metadata

The XcodeGen project now sets `MARKETING_VERSION` to `0.1.0` and `CURRENT_PROJECT_VERSION` to `1`. An isolated Release generic iOS Simulator build succeeded from the same Swift source blobs cited above with those project settings. The generated Info.plist reported `CFBundleShortVersionString = 0.1.0` and `CFBundleVersion = 1`; its universal arm64/x86_64 simulator executable SHA-256 was `6b0cf1f6104962322ceb09fdf59ecd064d6f0081780f0b5906b00fa4327bc4b8`. Xcode emitted an existing SwiftUI/Combine import warning. This was a build and metadata check; the new binary was not launched, and a device archive, team signing, icons, distribution and physical-device validation remain open. Earlier binaries were preserved.
