> Historical validation record. GPUI is excluded from the current build tree under the dependency-license policy. Its source and the original instructions remain in [checkpoint history](https://github.com/intrepion/kata-inspired-by-pong-codex-dot/tree/265a13eaff0ba638c49f10946b0633ed79d3f405). Results below describe earlier builds, not current availability.

# Native GUI-toolkit Pong comparisons

These standalone Cargo projects intentionally sit outside the root workspace so GUI dependencies and lockfiles cannot disturb the game-engine comparison. They share the existing deterministic game core through a small adapter with key-release/focus-loss tests. These are GUI experiments, not claims that a widget toolkit replaces a game engine.

| Variant | Renderer/platform notes | Verification |
| --- | --- | --- |
| `gui-slint` | Slint 1.18.1; winit backend and software rendering; requires Rust 1.92+ | Mac check + full build PASS; native rendering, Enter/W/S/R/Space and scoring/game-over exercised; Windows/Linux pending |
| `gui-gpui` | Official published GPUI 0.2.2; native GPU UI | Mac check PASSES with `--features gpui/runtime_shaders`; default build needs Xcode Metal Toolchain; binary/gameplay and Windows/Linux unverified |
| `gui-gtk` | GTK4 DrawingArea/Cairo; gtk-rs 0.10 | Omarchy aarch64 check + full build PASS; 8-second process smoke-launch clean (timeout124); interactive QA unverified. Mac missing GTK4/pkg-config; Windows unverified |
| `gui-support` | Input/fixed-step wrapper around pong-core | 4 unit tests passing on Mac and Omarchy |

## Native dependencies

Slint: native winit platform dependencies on Linux (Wayland/X11 development libraries); no Qt required. GPUI: official GPUI platform requirements apply, including GPU drivers and Linux font/window-system dependencies. GTK: GTK4 development headers and pkg-config must be installed. On macOS use official Homebrew packages `gtk4` and `pkgconf`; on Arch/Omarchy use `gtk4` and `pkgconf`; Windows needs the supported GTK4/MSYS2 development/runtime setup, not just Rust. Do not claim cross-platform availability until built and exercised on each host.

## Licenses / attribution

Our adapters are MIT under the repository LICENSE. This does not relicense their dependencies.

- Slint: used under **Slint Royalty-free License 2.0**, not automatically relicensing this application under GPL. The visible built-in `AboutSlint` component provides Slint attribution. Review the full Slint license before distributing any binary, especially for embedded uses: https://github.com/slint-ui/slint/blob/master/LICENSES/LicenseRef-Slint-Royalty-free-2.0.md
- GPUI 0.2.2: Apache-2.0. https://github.com/zed-industries/zed/tree/main/crates/gpui . Retain its license and applicable NOTICE files in binary packaging. The full Zed application's licensing is not the GPUI crate's licensing.
- gtk-rs Rust bindings: MIT. GTK itself: LGPL-2.1-or-later. https://www.gtk.org/docs/getting-started/ . Keep native libraries dynamically linked and include applicable notices/source or relinking obligations when distributing. Runtime redistribution on Windows/macOS needs explicit packaging work.
- Every transitive dependency retains its own license; no installers or release bundles have been produced by these experiments.

## Manual acceptance checklist

For each actual OS: launch, start, W/S and arrows, simultaneous/released input, pause/resume, reset, scoring/game-over, resize/DPI, focus loss/minimize, close cleanly. Native testing is through the app; any browser testing must use a local HTTP server, never file URLs.

### GPUI macOS shader toolchain

The default check fails in GPUI’s build script because `xcrun metal` requires Apple’s separately installed Metal Toolchain. Official command: `xcodebuild -downloadComponent MetalToolchain`. An alternative supported by pinned GPUI 0.2.2 is `--features gpui/runtime_shaders`, which moves shader compilation to application startup; that still requires runtime validation and is not a guarantee of launch success.

### GTK Omarchy verification

GTK4 4.24.1 was already installed. Cargo check and build -j1 passed with gtk-rs 0.10.3. The native executable remained alive for an 8-second smoke launch with no stderr, then was terminated by the test timeout (exit124). This establishes process startup only, not rendered appearance or interactive gameplay. Omarchy native computer-control APIs were unavailable for input/visual QA. No system package was installed.

### Slint macOS native validation

Built with CARGO_PROFILE_DEV_DEBUG=0 and CARGO_PROFILE_DEV_INCREMENTAL=false to preserve disk space. Native window inspected through Computer Use: visible court, score, controls and Slint attribution. Enter started; W/S moved paddle; R reset score; Space paused; CPU scored to seven and game-over displayed. Arrow/Escape and focus-loss UI paths are not yet verified (automation key naming), although shared adapter focus-loss and input release tests pass. Test process closed after inspection.


## Windows Slint follow-up (2026-11-12)

Locked full MSVC build passed with two jobs. Supported native Computer Use verified clean court/status/attribution rendering, Enter start, scoring/ball movement, Space pause/resume, R resetting scores to 0:0, and titlebar close. Sustained paddle movement/release, full-match win behavior, focus/minimize and resize/DPI remain unverified. No source changes were required. GPUI build is still in progress, not yet a pass.

## Windows GPUI follow-up (2026-11-12)

Locked full MSVC build passed (11m31s including earlier target-lock wait). GPUI dependency proc-macro-error2 emits a future-Rust-compatibility warning, not a build failure. Native supported Computer Use verified a readable court/status, Enter start, score advancement/ball movement, Space pause/resume, R reset to 0:0, maximized rendering and titlebar close. The court stretches with the window (ball becomes elliptical at wide aspect); no pixel/aspect-ratio fidelity claim is made. Sustained paddle movement/release, focus/minimize and complete matches remain unverified. Shared GUI adapter tests pass 4/4 on Windows. Both Slint and GPUI test windows are closed; no user files or build caches deleted.
