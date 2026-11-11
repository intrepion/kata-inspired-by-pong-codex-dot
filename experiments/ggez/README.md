# Pong / ggez

Independent experiment pinned to ggez 0.10.0, sharing `pong-core` with the other frontends. Uses ggez's current winit physical-key API and Canvas graphics. Audio/gamepad dependencies are disabled because this experiment does not use them.

```sh
cargo run --manifest-path experiments/ggez/Cargo.toml
```

W/S or arrows move; Enter starts/resumes, R restarts, Space toggles pause, Escape pauses. Losing focus pauses and clears held input. Resizing preserves the court aspect ratio. Simulation steps at 120 Hz and caps catch-up to 100 ms. Run from the repository root. Dependency lockfile is local to this experiment.

## Validation (2026-11-11)

Mac build and two surface-availability regression tests passed. Native gameplay verified: start/restart, keyboard paddle movement, ball movement, scoring, pause/resume, and window minimize/restore responsiveness. Test processes stopped. Windows gameplay remains unverified; see the Omarchy checkpoint below.

## Surface availability compatibility patch

The scoped `vendor/ggez` patch fixes ggez 0.10.0 repeatedly acquiring an occluded/timed-out surface inside the main event thread. This could starve visibility and accessibility events on macOS Metal. The fix defers drawing until the surface is available, preserving input cleanup and existing error handling. Occlusion checks remain enabled. See `vendor/ggez/PONG-PATCH.md` for provenance and upgrade/removal guidance.

```sh
cargo test --manifest-path vendor/ggez/Cargo.toml --no-default-features --lib pong_surface_regression_tests
```

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
