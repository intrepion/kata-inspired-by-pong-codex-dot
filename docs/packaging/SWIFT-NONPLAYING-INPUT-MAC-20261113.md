# Swift non-playing touch model, macOS

2026-11-13. Source files from checkpoint 42a7197d534d173b7537bb39cb638b0bf628124b were tested in an isolated overlay on macOS ARM64 with Apple Swift 6.4.2. A standalone compiled model harness passed: drags beginning in Ready, Paused and Over remained suppressed through start or resume, and a drag active at match end remained suppressed through the restart-button sequence. Ending the gesture and beginning a fresh playing drag moved the paddle in every case. The harness explicitly models the app's calls to `suppressActive`; it does not exercise SwiftUI gesture recognition, multitouch, simulator or physical devices.

The exact tested sources: Game.swift SHA-256 `affff1c44bd381bdca672668b3542561161467ca74d478c25c1465c8887703e2`, TouchInput.swift `8b6af0db4e425f2278bfad02b85c787f88d021736bf355e1b4dec48791a9ac15`, PongApp.swift `f5098ed647948cf05ddd0babe01bd42a52c6c1a0d6da12f17458bc35aab1b3b5`. The app file was inspected for callback order; it was not compiled or exercised by this harness. The checked-in harness SHA-256 is `ebea88204fb491b33beb0bee34168ba291c9b166e3dd91734afa662b6bb5d502`.

To reproduce with Swift, copy `scripts/swift-nonplaying-input-smoke.swift` to a temporary `main.swift`, then compile it together with `experiments/native-swift/Sources/PongCore/Game.swift` and `TouchInput.swift`, and run the resulting binary. No app build or release is implied.

`swift test` was attempted with isolated writable cache and scratch directories, but Swift Package Manager failed to initialize its nested sandbox with `sandbox-exec: sandbox_apply: Operation not permitted`. The package XCTest suite was therefore not verified in this run. Existing data and simulator state were preserved.
