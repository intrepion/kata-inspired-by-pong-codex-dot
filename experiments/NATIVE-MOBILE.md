# Native mobile comparison
These are deliberately independent implementations, not Rust FFI wrappers:
- `native-kotlin`: Kotlin model + Android Activity/custom Canvas View, Gradle/Android SDK.
- `native-swift`: Swift value-type model + SwiftUI Canvas, Swift Package tests and XcodeGen project.
- The former Tauri mobile WebView implementation is excluded under the dependency-license policy; its source and historical results remain in Git history.

Compare `crates/pong-core/src/lib.rs` with Kotlin `Game.kt` and Swift `Game.swift`. Each keeps 900x500 coordinates, 90-unit paddles, 470 player speed, first-to-seven, .65-second serves, .3-second CPU reaction, 190/130 CPU speed, angle shots, 1.055 acceleration and 720 ball-speed cap. The same xorshift64 seed algorithm is ported. Floating-point trig/rounding can differ by platform: these are rule-equivalent, not certified bit-identical replays.

Native apps use drag targets with speed-limited paddle movement rather than teleporting to the finger. Backgrounding pauses, and returning requires Resume. Restart intentionally resets the match. Rotation currently recreates Android Activity and therefore resets the match; Swift view state survives ordinary layout rotation. Persistence, sound, accessibility of the game field, controller support and store signing are outside this first comparison.

## Run Swift on Mac
```sh
cd experiments/native-swift
swift test --jobs 2
xcodegen generate
xcodebuild -project PongNativeSwift.xcodeproj -scheme PongNativeSwift -sdk iphonesimulator -destination 'generic/platform=iOS Simulator' -derivedDataPath DerivedData CODE_SIGNING_ALLOWED=NO build
# Substitute an available simulator UUID:
xcodebuild -project PongNativeSwift.xcodeproj -scheme PongNativeSwift -destination 'platform=iOS Simulator,id=SIMULATOR_UUID' -derivedDataPath DerivedData CODE_SIGNING_ALLOWED=NO test
```
Nine core tests and start/pause/resume/restart XCUITest passed on Mac, iPhone16/iOS18.4 simulator, Xcode27.2 SDK. App installed/launched and ready-screen pixels inspected. Full-match touch gameplay and physical-device testing remain separate checks.

## Run Kotlin on Windows
Set JAVA_HOME to JDK21 and ANDROID_HOME to the Android SDK, then:
```powershell
cd experiments/native-kotlin
.\gradlew.bat --no-daemon --max-workers=2 testDebugUnitTest assembleDebug
& "$env:ANDROID_HOME/platform-tools/adb.exe" install -r app/build/outputs/apk/debug/app-debug.apk
& "$env:ANDROID_HOME/platform-tools/adb.exe" shell am start -n com.intrepion.pong.nativekotlin/.MainActivity
```
Uses AGP9.3.1 built-in Kotlin and Gradle9.6.1. No Rust compiler or JNI bridge is required for this variant. Nine tests mirror core rule scenarios. Build artifacts are ignored; no app-store distribution or publication is configured.

## Tradeoff
Independent ports make language idioms, native UI/lifecycle APIs and debugging easy to compare, but every rule change must be applied/tested three times. A Rust bridge would centralize correctness but hide much of the Kotlin/Swift comparison. Keep the shared Rust version as the long-term baseline and these as focused educational experiments.

## Final verification (2026-11-11)
Windows: nine JUnit tests passed, debug APK built/installed on Android15/API35. Actual touch swipe moved paddle; ball movement, pause/resume, restart verified with screenshots. System-bar overlap found and fixed; final rebuild/tests passed. Mac: nine Swift core tests and one XCUITest covering start/pause/resume/restart passed. No physical-device or full manually played-match test. Apps remain local, uncommitted and unpublished.
