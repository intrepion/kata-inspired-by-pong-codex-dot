# Pong / miniquad

Isolated Rust rendering experiment using the repository's deterministic `pong-core`. It does not alter the root workspace.

```sh
cargo run --manifest-path experiments/miniquad/Cargo.toml
```

Controls: W/S or arrows move; Enter or R starts/restarts; Space starts or pauses/resumes; Escape exits. Fixed 120 Hz simulation with bounded catch-up, a first-to-seven CPU opponent, and aspect-preserving resizing.

This is an experiment, not a separately published product. Native execution is not a browser/file URL test.

Uses Miniquad's default OpenGL backend, not Macroquad. Score pips replace a font dependency. Minimizing pauses and clears input; general focus-loss behavior needs platform QA.

## Latest Mac checkpoint

Mac check/build and renderer-coordinate test passed. Native start, movement, scoring and pause verified. Windows/Omarchy remain unverified.

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
