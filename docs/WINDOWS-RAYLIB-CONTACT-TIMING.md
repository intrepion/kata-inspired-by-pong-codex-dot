# Windows raylib contact-timing recheck

On 2026-11-13, raylib-rs was built from `1d83e7401d730e0d5a469019d10e06d269e9a87c` with locked offline dependencies in a fresh Windows MSVC debug target. Three adapter tests passed. Build and test links emitted `LNK4098` for conflicting `MSVCRTD` default libraries; this warning was not suppressed or investigated in this run. Executable SHA-256: `19b4b2360c3601037995b5d180cc62c6c634c96c047f019962511b0e1692b968`.

The scripted replay exited 0 at tick 1440 with empty stderr and six app-owned PNGs. Its 13 [telemetry records](evidence/windows-raylib-contact-timing-20261113.jsonl), filtered from raylib's mixed stdout, matched the contact-timing model reference line-for-line. The paused tick 480 capture was inspected and shows the court, score, paddles, ball and pause text. This is an app-owned capture, not a desktop screenshot or proof of OS input.

No new normal interactive test was performed. The earlier Windows supported start-input limitation remains open; physical keyboard, full match, focus/resize, packaging and release qualification remain unverified for this revision. Previous binaries and captures remain historical.

## Runtime warning investigation

The generated raylib CMake Debug project selected `MultiThreadedDebugDLL`; `dumpbin /directives` showed `MSVCRTD` in `raylib.lib` and `MSVCRT` in the separately compiled raymath shim. The executable imported the non-debug CRT. This independently identifies a runtime mismatch, rather than inferring the SDL3 result applies to raylib.

An isolated probe used the repository's existing explicit MSVC CMake toolchain file, which selects `MultiThreadedDLL`, with a new target directory. The debug build and three tests passed without `LNK4098`; `raylib.lib` contained 27 `MSVCRT` directives and no `MSVCRTD` directives. Probe executable SHA-256: `c21dffb2dca68712c4dfc9193e887d9b775e629a4f0db59a92f46b573781fe30`. Its scripted replay exited 0 with empty stderr, six app-owned PNGs and 13 telemetry records matching the reference line-for-line. This is not a native-input or package pass.

The explicit toolchain setting is a probe, not an automatic fix for ordinary Cargo. The cached `raylib-sys` build script offers no identified raylib-specific runtime selector; changing it would require a maintained dependency patch, while a global CMake setting could affect unrelated builds. Neither was adopted in this bounded investigation. The ordinary debug warning remains open and no linker diagnostic was suppressed.
