# Pong / raw wgpu + winit

No game engine: winit owns the window/input loop, wgpu owns GPU presentation, and the existing `pong-core` owns gameplay. `renderer.rs` keeps graphics separate from `main.rs`; `court.wgsl` draws a resolution-independent court from a small state uniform. There is no pixels, Macroquad, Bevy or GUI toolkit dependency.

```sh
cargo run --manifest-path experiments/wgpu/Cargo.toml
```

W/S or arrows move, Enter/R starts or restarts, Space starts/pauses/resumes, Escape closes. Focus loss pauses and clears inputs. Fixed 120 Hz simulation and bounded catch-up. Score pips appear in court and numeric score/state in the window title. Resizing preserves the court aspect ratio. Zero-size windows skip surface resizing; lost/outdated surfaces are reconfigured.

This isolated experiment does not modify the root workspace. Native tests are run as applications, never through file URLs.

## Latest Mac checkpoint

Mac cargo check and full build passed. WGSL pipeline rendered correctly; native movement, scoring, pause and restart verified. Windows/Omarchy remain unverified.

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
