# Pong / Fyrox

Independent experiment pinned to Fyrox 1.0.1 using its native Executor/Plugin lifecycle and built-in UI renderer, not a browser. The same `pong-core` drives the simulation. This is intentionally a small 2D comparison, not a demonstration of Fyrox's scene editor or 3D capabilities.

Run from the repository root:

```sh
cargo run --manifest-path experiments/fyrox/Cargo.toml
```

W/S or arrows move; Enter starts/resumes, R restarts, Space pauses, Escape pauses. Focus loss pauses and clears held input. Court scales with the window; fixed 120 Hz simulation, bounded catch-up. The full engine requires more dependencies/build time than the lightweight experiments. Validation status is recorded separately; source presence is not a claim of successful runtime testing.

## Validation (2026-11-11)

Mac cargo check and full build passed. Fixed missing initial UserInterface startup panic and zero-sized wrapping Canvas clipping. Native court, movement, scoring/game-over, restart and pause verified. Windows/Omarchy untested; focus and resize handling implemented but not exhaustively exercised.

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
