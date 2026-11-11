# Pong / ggez

Independent experiment pinned to ggez 0.10.0, sharing `pong-core` with the other frontends. Uses ggez's current winit physical-key API and Canvas graphics. Audio/gamepad dependencies are disabled because this experiment does not use them.

```sh
cargo run --manifest-path experiments/ggez/Cargo.toml
```

W/S or arrows move; Enter starts/resumes, R restarts, Space toggles pause, Escape pauses. Losing focus pauses and clears held input. Resizing preserves the court aspect ratio. Simulation steps at 120 Hz and caps catch-up to 100 ms. Run from the repository root. Dependency lockfile is local to this experiment.

## Validation (2026-11-11)

Mac cargo check and full build passed, including the 120 FPS pacing patch. Process launches, but repeated native inspection timed out. Process sample was in Metal surface acquisition; the cause is not established. Gameplay is NOT verified. Windows/Omarchy untested. Test processes stopped.

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
