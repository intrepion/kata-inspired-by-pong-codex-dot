# Pong / nannou

Isolated Rust rendering experiment using the repository's deterministic `pong-core`. It does not alter the root workspace.

```sh
cargo run --manifest-path experiments/nannou/Cargo.toml
```

Controls: W/S or arrows move; Enter or R starts/restarts; Space starts or pauses/resumes; Escape exits. Fixed 120 Hz simulation with bounded catch-up, a first-to-seven CPU opponent, and aspect-preserving resizing.

This is an experiment, not a separately published product. Native execution is not a browser/file URL test.

nannou 0.20 uses Bevy internally, so this compares creative-coding APIs rather than an entirely separate engine. Focus loss pauses the match.

## Validation (2026-11-11)

Mac cargo check and full build passed. Native court, movement, scoring, pause and restart verified. Initial launch/inspection was slow; default Mailbox fell back to Immediate and CPU usage was high. Windows/Omarchy untested.

## Omarchy ARM64 checkpoint

Full build and short native host launch smoke test passed on 2026-11-11. This is not interactive gameplay verification: Computer Use exposed no native surfaces on that device. Sources were tested in a separate validation folder; no checkout overwrite or publication.
The 120 Hz update-rate limit compiled and launched on Omarchy. Mac validation above predates this pacing change; CPU improvement is not yet measured.
