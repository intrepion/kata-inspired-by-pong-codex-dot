# Windows SDL3 contact-timing recheck

On 2026-11-13, SDL3 was freshly built with locked offline dependencies in a separate Windows MSVC debug target from `617160ceb4573ff06f7339c7dbc98787d54f55df`. Both SDL3 adapter tests passed. The build and test links emitted `LNK4098` about conflicting `MSVCRTD` default libraries; this warning was not suppressed. Executable SHA-256: `299eb8a88e5ffcb9495b568b80e87723dac4a15a388f91d4e0290bcec122e723`.

The scripted replay exited 0 at tick 1440 with empty stderr, 13 [telemetry records](evidence/windows-sdl3-contact-timing-20261113.jsonl), and six app-owned PNGs. Discrete fields matched the contact-timing model reference and maximum absolute numeric difference was zero (allowance 0.001). The paused tick 480 capture was inspected and shows the court, score, paddles, ball and pause marker. This renderer capture is not a desktop screenshot or proof of OS input.

No normal interactive test was performed in this follow-up. Physical keyboard, native input, full match, focus/resize, package and clean-machine qualification remain open for this revision. The linker warning warrants separate investigation before release. Previous binaries and captures remain historical evidence.

## MSVC runtime follow-up

The warning was reproduced in a fresh debug build. The generated SDL Debug project selected `MultiThreadedDebugDLL` and its static library contained `MSVCRTD` directives, while the linked Rust executable imported the non-debug CRT. An environment-only CMake runtime setting did not change the generated Debug setting and the warning persisted. An independent release build selected `MSVCRT`, linked without this warning, and passed both adapter tests.

For Windows MSVC, `scripts/build-sdl3-windows.ps1 -Action build` and `-Action test` provide an opt-in debug build path that sets SDL's CMake runtime to `MultiThreadedDLL`, matching Rust's default dynamic runtime. The script uses a separate target directory and refuses to replace an existing `CMAKE_TOOLCHAIN_FILE`. It does not suppress linker diagnostics or change the ordinary Cargo command. The stable MSVC Rust toolchain and CMake build prerequisites are required.

A fresh build through this path linked without `LNK4098`; `dumpbin` found 268 `MSVCRT` and zero `MSVCRTD` directives in SDL's static library. Both adapter tests passed. The resulting debug binary SHA-256 was `9af9c308d472c1bcd7d26564279d10f672a265d18142af61b5f893b0467b965f`; its scripted replay exited 0, produced six app-owned PNGs and empty stderr, and its 13-record telemetry was byte-identical to the earlier Windows contact-timing replay. This is not a native-input, package, or clean-machine pass. The ordinary debug Cargo path still emits the warning; no global setting or prior artifact was changed.

### Automatic-path investigation

A separate uncommitted experiment added `[profile.dev.package.sdl3-sys] opt-level = 1` to the SDL3 workspace. A fresh ordinary locked debug build linked without `LNK4098`, but CMake selected `RelWithDebInfo` for SDL rather than Debug. This profile applies on all platforms and changes SDL's debug optimization and assertion configuration, so it was not adopted as a Windows-only runtime fix. The experiment and its target remain isolated; the opt-in toolchain path above remains available. A narrowly scoped automatic fix would require a supported target-specific SDL build configuration or a maintained dependency change; the regular debug command remains unresolved.
