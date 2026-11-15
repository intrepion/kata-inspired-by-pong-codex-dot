# ash Vulkan present-request callback timing

Opt-in `--benchmark` uses the shared seeded scenario and sampler in the existing native Vulkan renderer. A sample is recorded only after successful `queue_present`; null swapchains and out-of-date recovery without a successful present request are not counted. The renderer now returns separate `resize` and `presented` flags, preserving its existing recovery behavior.

The existing FIFO swapchain mode, serialized `queue_wait_idle` baseline and 8 ms winit WaitUntil remain unchanged. This measures callback intervals with those deliberate constraints, not GPU duration or displayed FPS. It is not a ranking against engines with other callback boundaries or synchronization strategies.

Simulation uses 120 Hz steps and a 100 ms catch-up cap; normal focus/input is excluded only in benchmark/replay mode. Replay/capture flags and benchmark mode are mutually exclusive. No capture readback or per-tick JSON occurs during measurement. The report prints once and requests normal event-loop exit; failures retain nonzero exit status.

Build: `cargo build --release --offline --locked --manifest-path experiments/ash/Cargo.toml`

Run: `node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter ash`

The protocol requires three valid bounded release repeats, each five seconds warm-up plus ten seconds measured. Platform Vulkan loader/driver requirements are not removed by this adapter; a build is not runtime qualification. Native real-input/focus and packaging/install tests remain separate.

## Omarchy ARM64, 2026-11-12

Four Rust tests, twenty shared Node tests, strict Clippy and release build pass. [Three valid repeats](ash-omarchy-20261112.json) exit normally: 766/782/780 samples, medians 13.699/12.958/13.588 ms, 1474×1864. Original private reports remain local; the published copy explicitly allowlists timing/provenance. Source revision identifies the pre-publication base; this commit adds the adapter used to produce the recorded binary SHA.

A separate native replay exits 0 with thirteen states through tick 1440/finished=true and six application Vulkan framebuffer PNGs. Tick600 was inspected: score 0:1, two paddles, ball and dashed net are visible. No new OS input/focus or packaging claim. Existing resize recovery remains intact but was not deliberately induced during timing.

Mac's previously recorded missing Vulkan/MoltenVK loader remains a runtime blocker until rechecked there. This Omarchy result does not qualify Mac or Windows.
