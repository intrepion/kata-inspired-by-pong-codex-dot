# Android app-owned scripted replay

Debug builds accept an explicit Activity intent extra `pong_scripted=true`.
Release builds ignore it. Ordinary launches keep existing touch/button controls.

The app reads the packaged canonical manifest, advances one deterministic logical
tick per visible frame, and renders with the same Canvas drawing function as normal
play. It records JSONL and six PNG checkpoints in a new UUID directory under app
private `files/replays/`. Captures include only this app's game view, not the OS,
other applications, title or buttons. Normal controls are disabled during replay.
Backgrounding suspends replay drawing; finishing stops the animation loop.

```powershell
adb -s emulator-5554 install -r app/build/outputs/apk/debug/app-debug.apk
adb -s emulator-5554 shell am start -S -n com.intrepion.pong.nativekotlin/.MainActivity --ez pong_scripted true
adb -s emulator-5554 shell run-as com.intrepion.pong.nativekotlin ls files/replays
```

Read the resulting app-owned files through the debug app's `run-as` access. Each
completed directory contains `complete.json`, `telemetry.jsonl` and checkpoint
PNGs. These are renderer/replay evidence, not physical touch/focus validation.
No test deletes prior runs or changes emulator security/data.

## Current verification

Windows Gradle build and all 11 JVM tests passed after wiring. Emulator runtime is
pending: installation returned `Can't find service: package`; supported UI inspection
showed the emulator still at the Android boot logo, not an active user app. A routine
no-wipe reboot was requested. No installed/replayed/captured success is yet claimed.
