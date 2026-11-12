# Fyrox pre-render callback timing

Opt-in `--benchmark` repeats the shared seeded scenario and samples native `before_rendering` callbacks. Pinned Fyrox 1.0.1 calls this hook immediately before `engine.render()`. This is a pre-render callback interval, not GPU completion or displayed FPS. The existing desired update rate of 120 Hz and renderer/window defaults are unchanged.

Native physical window dimensions are recorded; a resize during the measured interval invalidates that run. Normal game input/focus handling is skipped only in benchmark/replay mode. Shared scenario advances in 120 Hz steps using the existing capped update delta. Timing and scripted replay/capture flags are mutually exclusive. On completion the report prints once and requests the existing loop controller exit.

Build: `cargo build --release --offline --locked --manifest-path experiments/fyrox/Cargo.toml`

Run: `node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter fyrox`

The common pilot requires three normal exits, five seconds warm-up and ten seconds measurement each. No screenshot readback or per-tick JSON occurs in timing mode. Different machine and callback/pacing boundaries do not support a controlled engine ranking. Native input/focus, captures and package lifecycle are separate evidence categories.

## Omarchy ARM64, 2026-11-12

Two Rust mode tests, nineteen shared Node tests, strict Clippy and offline release build pass. [Three timing repeats](fyrox-omarchy-20261112.json) all exit 0 with 1200/1198/1198 samples; medians 8.332/8.332/8.331 ms at 1474×1864. Raw private host/environment information remains local, with explicit allowlisted publication. Recorded base revision plus this commit's adapter changes produced the binary identified by SHA in the report.

A separate bounded native replay exits 0 with thirteen checkpoint states through tick 1440/finished=true and six app-owned UI framebuffer PNGs. Tick600 was visually inspected: court, paddles, ball, centerline and score/instructions are present. Portrait output is letterboxed; the existing header uses small text rather than matching court scaling. This is not a real-input/focus or packaged-install test, nor a claim of pixel identity across backends.
