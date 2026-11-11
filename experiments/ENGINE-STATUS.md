# Additional engine experiments

These isolated Cargo workspaces share `../../crates/pong-core`; they are not root workspace members. Comfy and rend3 were excluded at the user's request. No changes here have been published.

## Mac Apple Silicon validation (2026-11-11)

| Frontend | Compile status | Native execution status |
|---|---|---|
| Miniquad 0.4.11 | Check, build, coordinate unit test pass | Start, paddle movement, scoring and pause verified |
| pixels 0.17.2 / winit 0.30.13 | Check, build, framebuffer unit test pass | Start, movement, scoring, pause and restart verified |
| raw wgpu 29.0.4 / winit 0.30.13 | Check and build pass | Movement, scoring, pause and restart verified |
| Piston 0.148.0 | Check and build pass | Court, keyboard movement, start, pause and restart verified |
| ggez 0.10.0 | Check and build pass | Process launches; native inspection repeatedly times out. Gameplay NOT verified |
| nannou 0.20.0 | Check and build pass | Court, movement, scoring, pause and restart verified after slow initial startup |
| Fyrox 1.0.1 | Check and build pass | Court, movement, scoring/game-over, restart and pause verified |
| Blue Engine 0.10.0 | Check and build pass | Court, movement, scoring, pause and restart verified |
| raylib-rs 6.0.0 | Check, build, viewport unit test pass | Movement, scoring, pause and restart verified |

Windows validation is underway. Omarchy ARM64: all nine full builds passed and all nine sustained native launch smoke tests (8 seconds each, nannou 12 seconds) without crashes. Interactive gameplay is unverified there because Computer Use returned no native surfaces. Miniquad/pixels/raylib adapter tests passed. Other workers' GUI, terminal, web and low-level experiments have independent reports. Build success does not imply all-platform support or exhaustive UI QA.

## Notes

- nannou 0.20 is Bevy-based. First startup was slow and its unsupported Mailbox present mode fell back to Immediate on this Mac; CPU usage was high. Explicit frame pacing is a follow-up, not yet verified.
- Fyrox required an initial UserInterface and removal of an unsized wrapping Canvas to avoid a startup panic and invisible court. Its large default window/layout remains basic.
- ggez was sampled in the Metal surface-acquisition render loop. A rebuilt 120 FPS pacing patch did not resolve native inspection timeouts; cause remains unconfirmed. Test processes stopped.
- Blue Engine uses absolute resize rather than cumulative scaling; zero score bars are hidden rather than scaled to a singular matrix.
- CMake 4.4.5 was installed from official Homebrew core for raylib and sibling SDL3 builds.
- Compilation was serialized. Approximately 11 GiB free disk remained at the final checkpoint; no user data or caches deleted.

## Cross-platform pacing follow-up

nannou now calls `app.set_update_rate(120.0)` to limit work when presentation is non-blocking. This revision compiled and launched on Omarchy; Mac results above predate this tiny patch. CPU improvement has not been measured. Omarchy used an isolated validation folder, shared 5.8 GiB target, one build job and no debug/incremental artifacts; approximately 288 GiB remained free. No user data deleted.

## Windows interactive follow-up (2026-11-11)

Used existing Windows GNU executables from the isolated extra-engine validation snapshot, with supported Computer Use only. These binaries predate the later macOS-specific vendored ggez patch; this pass does not certify a fresh build from the latest checkpoint. Every test window was closed and absence verified afterward. No unrelated applications were closed.

| Variant | Observed native result |
|---|---|
| pixels | Clean court; Return started; ball/CPU motion and scores advanced; Space paused/resumed; Return restarted into a new match. |
| raw wgpu | Clean court; Return started; scoring/ball motion, Space pause/resume, Return score reset passed. |
| Piston | Clean court; Return start, scoring/ball motion, Space pause/resume, R reset to 0:0 passed. |
| Blue Engine | Clean court; Return start, scoring/ball motion, Space pause/resume, R reset to 0:0 passed. |
| Fyrox | Clean court; Return start, scoring/ball motion, Space pause/resume, R reset to 0:0 passed. |
| nannou | Clean court; Return start, scoring/ball motion, Space pause/resume, Return reset to 0:0 passed. |
| ggez | Clean court; Return start, scoring/ball motion, Space pause, Return resume, R reset to 0:0 passed. No inspection timeout on Windows. |
| Miniquad | Clean initial court, but Return did not start visible play, including after clicking the court to focus. Input/gameplay remains unverified; this is not a crash. |

These are bounded interactive checks, NOT full gameplay acceptance. Sustained paddle movement/release, complete matches, resize, focus-loss/minimize, and other physical input remain unverified in this Windows engine pass. A brief W press in pixels/raw-wgpu produced no clearly visible paddle displacement; the native automation API does not support durationMs holds, so no movement success is inferred. Other engines' paddle movement was not attempted here. Browser and WebView pointer tests are recorded separately.

Raylib's separate LLVM-unblock pass is recorded in the Windows validation notes: build/unit test/clean Ready rendering passed, but supported Enter/Space did not start play. Neither Miniquad nor raylib should be labeled a gameplay pass merely because a window rendered.
