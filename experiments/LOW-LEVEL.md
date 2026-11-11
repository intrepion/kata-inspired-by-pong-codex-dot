# Low-level Pong comparisons

These independent Cargo workspaces all use the repository's `pong-core`, plus a small dependency-free `low-level-common` adapter for fixed 120 Hz simulation, identical rectangle layouts, score digits, and letterboxing. They do not modify the original browser game.

| Folder | Rendering path | What this comparison teaches |
| --- | --- | --- |
| `softbuffer` | CPU rasterization + winit native window | Direct pixel buffers, no GPU rendering framework |
| `sdl3` | SDL3's cross-platform 2D Renderer through Rust bindings | Native multimedia library, rendering abstraction, source/static linking |
| `ash` | Vulkan via ash + ash-window + winit | Device/queue selection, swapchain, render passes, explicit synchronization and destruction |

Run from the repository root:

```sh
cargo test --manifest-path experiments/low-level-common/Cargo.toml
cargo run --manifest-path experiments/softbuffer/Cargo.toml --jobs 1
cargo run --manifest-path experiments/sdl3/Cargo.toml --jobs 1
cargo run --manifest-path experiments/ash/Cargo.toml --jobs 1
```

Controls: W/S or Up/Down; Space starts or toggles pause; R restarts; Escape quits. Focus loss clears held input and pauses. First to seven wins. Scores, state, and control help also appear in the window title. The orange pause bars indicate Ready, Paused, or Over. All three draw the same rectangular ball to compare rendering rather than gameplay.

## Dependencies and platform limits

- **softbuffer:** native windowing libraries required on Linux (Wayland/X11), ordinary OS support on Mac/Windows. CPU drawing is not a no-dependency implementation.
- **SDL3:** The default `bundled` Cargo feature enables `build-from-source-static`, compiling the upstream SDL3 source through its crates.io source crate. CMake and a C/C++ toolchain are needed; on Windows use the MSVC build tools, on Mac Xcode command-line tools. Static SDL still depends on OS frameworks. `--no-default-features` instead uses a system SDL3 library; a Rust-only `cargo check` works in that mode without linking SDL3. This does not install a global SDL runtime.
- **ash:** dynamically loads the Vulkan loader. Windows/Linux need a Vulkan-capable driver/loader. **macOS needs MoltenVK and a Vulkan loader/ICD configuration**, typically via the official LunarG Vulkan SDK. Merely compiling the Rust crate does not establish runtime support. No SDK has been installed by this experiment. Missing loader errors explain the requirement. Vulkan portability enumeration/subset extensions are enabled when advertised.

The ash baseline deliberately draws rectangles with `vkCmdClearAttachments` inside a Vulkan render pass. It is a real Vulkan rendering path, **not OpenGL or wgpu**, but does not yet demonstrate vertex buffers or shaders. It serializes frames with `queue_wait_idle` to make the simple semaphore lifecycle correct. This is a correctness-first learning baseline, not a performance benchmark or production renderer. A later extension could use shaders/vertex buffers and pipelined frames.

## Verification record

- Mac: low-level-common's three tests pass; all three manifests resolve dependencies.
- Mac: `cargo check` and full `cargo build` pass for softbuffer and ash, jobs=1. Binaries are in `/tmp/pong-pixels-target/debug/`.
- softbuffer native interactive QA passes using a temporary macOS app wrapper: visible board/ball/score, Space start/pause, paddle movement, R restart, focus-loss pause, and zoom/resized rendering.
- ash startup fails with an explicit missing `libvulkan.dylib` message, confirming the Mac runtime prerequisite. No Vulkan gameplay has been tested. The Cocoa `block 0.1.6` dependency also emits a future-Rust-compatibility warning.
- SDL3 default bundled full build passes after CMake was installed through the official Homebrew formula by the engine worker. Interactive QA passes: visible game rendering, Space start/pause, paddle movement, scoring, R restart, focus-loss pause, and zoom/resized rendering. The alternative system-SDL3 Rust-only check (`--no-default-features`) also passes. Binary: `/tmp/pong-sdl3-target/debug/pong-sdl3`.
- Windows and Omarchy compile/runtime: not run for these variants yet.
- Mac usual Homebrew, `/usr/local`, and `~/VulkanSDK` locations showed no Vulkan loader/MoltenVK installation. This is a runtime prerequisite to confirm, not a compile failure.

Do not interpret source availability or dependency resolution as a working application. Native gameplay, resize, focus loss, pause/restart, and close need testing separately on each platform. Browser comparisons elsewhere in the repo must be served over localhost, never `file://`.

## Cross-platform follow-up (2026-11-11)

### Omarchy

Isolated validation snapshot: `/home/intrepion/Code/pong-low-level-validation`, leaving the active checkout unchanged. Source archive SHA256: `4343b9ba5c6549a2d81b713814875c0e3e9b8fd73dd48639e01d1d6e0beb1706`.

- Shared adapter: 3 tests pass.
- softbuffer, SDL3, ash: full native builds pass, jobs=1.
- All three launch and run without errors for five-second smoke tests; timeout exit124 is expected. ash reports Vulkan device `Apple M1 Pro (G13S C0)`.
- Interactive visual/gameplay QA remains unavailable: Computer Use inventory contains no apps or browsers. Launch checks do not establish input or visual correctness.
- Build cache 1.6GiB; approximately 286GiB disk free. No system installs required.

### Windows

- Existing checkout does not contain these new clients (its experiments directory contains only native-kotlin).
- Source transfer through the current Library helper fails on native Windows Python: `AttributeError: module 'os' has no attribute 'setxattr'`. The helper aborts before installing the source archive. No fallback bypass or user-file overwrite was attempted.
- Prerequisites observed: Rust1.99.0 windows-gnu, CMake, Vulkan DLL; approximately195GiB free disk.
- These clients remain **not built, not launched, and not interactively tested on Windows**. The supported transfer must be fixed or the source made available through an authorized repository route first. No source publication has been authorized/performed here.

## Windows verified follow-up (2026-11-11)

The earlier source-transfer blocker is resolved: the approved Git checkpoint was fetched into a separate worktree without overwriting the original Windows checkout. Using stable-x86_64-pc-windows-msvc and two jobs, all three locked full builds passed from checkpoint 797055d. Shared low-level adapter tests passed 3/3. SDL3 bundled build succeeded with linker warning LNK4098 (MSVCRTD conflicts with other libraries); the warning remains, and this pass is not a release-linkage audit.

Through supported Computer Use, **softbuffer, SDL3, and ash** each visibly rendered the complete court, responded to Space start/pause/resume, advanced the ball and score, reset scores to 0:0 with R, and exited with Escape. Each window's disappearance was verified. No source edits, driver/SDK installs, security changes, or synthetic input outside the supported tool were needed. Softbuffer's initial launch approval timed out, then a supported retry succeeded.

These are bounded interactive passes, not full acceptance: sustained paddle movement/release, focus-loss/minimize, resize/DPI, complete matches, and release packaging remain unverified in this Windows pass. In particular, ash's successful simple clear-attachment rendering here does not establish that another Vulkan application or shader pipeline works on this GPU.
