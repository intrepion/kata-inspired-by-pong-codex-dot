# Windows Kotlin non-playing input regression

At checkpoint `ae94e8afabc2c852f55334d29cd1ead5de3098c6`, including `68bed13`, the native Kotlin `TouchInputTest.kt` source blob was `0a55fdfcbb0a9d2a8260fc3714cd0cae5fc3667f`. On Windows, an isolated project copy ran Gradle 9.6.1 offline with Microsoft JDK 21 and the installed Android SDK: `:app:testDebugUnitTest --tests com.intrepion.pong.nativekotlin.TouchInputTest` built successfully, with six tests, zero failures or skips. This includes `gestureBegunOutsidePlayRequiresFreshDownAfterStartOrResume`. The Kotlin compiler reported existing deprecations for the top and bottom system-window-inset APIs.

The initial sandboxed attempt could not connect to Gradle's local daemon; an authorized run with local daemon access initially lacked the Android plugin in the selected offline cache. A workspace copy of the existing Gradle cache supplied it for the successful offline run. The isolated project and test XML remain under `android-nonplaying-windows-20261113`; the copied cache remains under `android-nonplaying-gradle-cache-20261113`.

This is a JVM unit test of touch ownership and game transitions, not Android runtime, emulator, physical multitouch, accessibility, clean-machine or release qualification.
