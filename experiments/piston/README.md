# Pong / Piston

Independent experiment pinned to piston_window 0.148.0, sharing `pong-core`. This current Piston version uses wgpu/winit, not the OpenGL backend in many older tutorials.

```sh
cargo run --manifest-path experiments/piston/Cargo.toml
```

W/S or arrows move; Enter starts/resumes, R restarts, Space toggles pause, Escape pauses. Losing focus pauses and clears held input. The window title displays numeric scores, state and controls; court score bars work without external fonts. Resizing letterboxes the court. Simulation steps at 120 Hz and caps catch-up to 100 ms. Run from the repository root. Dependency lockfile is local to this experiment.

## Validation (2026-11-11)

macOS Apple Silicon: `cargo check` passed with Rust 1.99.1. Full executable build and GUI gameplay remain pending; Windows and Linux are unverified. Full build was deferred to release the Mac compilation slot for another task.

## Latest Mac checkpoint

Mac full build passed. Native court, movement, start/pause/restart verified. Windows/Omarchy remain unverified.

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
