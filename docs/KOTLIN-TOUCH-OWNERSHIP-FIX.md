# Kotlin Android touch ownership correction

The native Kotlin view previously read pointer index zero for every movement event and did not handle ACTION_POINTER_UP. With two fingers, lifting the controlling finger could retain a stale target or make an unrelated finger steer the paddle.

The new pure TouchInput model owns one pointer ID until that finger lifts. Secondary fingers cannot replace or release its target. The Android view uses actionIndex for pointer down/up and findPointerIndex for movement, clears missing/cancelled pointers, and clears ownership on Activity pause. A finger already down does not inherit movement after the owner lifts; a new down can take ownership. Scripted mode remains isolated from touch input.

Three tests cover secondary-finger interference, owner release while another finger remains, and cancellation/lifecycle clear with stale subsequent events. Windows offline Gradle testDebugUnitTest and assembleDebug succeeded: fourteen JUnit tests total (nine game, two scenario, three touch), zero failures/errors. [Build identity and evidence](evidence/kotlin-touch-windows-20261112.json). Two existing WindowInsets deprecation warnings remain.

This is model regression plus Android source/APK compilation, not actual MotionEvent instrumentation, emulator touch, physical multi-touch, installation or release acceptance. No OS settings, emulator state or installed application were changed. The debug APK stays local; it was not published as a release.
