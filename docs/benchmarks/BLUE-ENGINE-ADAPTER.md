# Blue Engine native update callback timing

`--benchmark` repeats the shared seeded scenario in the **native windowed build only**. The callback is sampled after object-state changes but before the framework's render submission. In pinned Blue Engine0.10, object upload precedes the user update callback, so object changes are rendered on a later framework pass. This measurement is neither GPU completion nor displayed FPS.

Existing AutoVsync request, camera/layout and normal rendering order are unchanged. The sampler records configured surface dimensions and rejects resize during measurement. Shared simulation retains120Hz fixed steps and100ms catch-up cap. Normal gameplay input/focus changes are skipped in benchmark mode; replay/capture flags are rejected. Completed timing prints once and uses Blue Engine's supported close_engine request, not process termination.

The headless capture build explicitly rejects benchmark mode. A headless export is not a native-window timing/visual substitute. No screenshot readback or per-tick telemetry occurs during native measurements.

`cargo build --release --offline --locked --manifest-path experiments/blue-engine/Cargo.toml`

`node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter blue-engine`

Common bounded protocol:5seconds warm-up,10seconds measured,3repeats,30second process limit. Driver and actual display pacing remain unqueried; default pacing/different callback boundaries do not support engine rankings.

## Omarchy ARM64 evidence, 2026-11-12

Two Rust tests, eighteen shared Node tests, strict Clippy and native release build pass. The separate headless feature compiles, but no headless timing or screenshot result is claimed. Native replay exits 0 with thirteen states through tick 1440/finished=true; that existing replay exit path is not a graceful-shutdown test.

The first timing run completed its measurement but exited abnormally: [retained invalid report](blue-engine-omarchy-initial-20261112.json). A second diagnostic reproduced SIGSEGV. The core stack places the fault in Wayland/EGL cleanup from wgpu resource destruction during Engine drop. Blue Engine's window field precedes its remaining object/camera resources. Holding a native window Arc until explicit Engine destruction avoids destroying the display connection before those resources. No forced process exit, backend substitution, or driver/system setting change was used for the timing fix. This evidence supports a teardown-lifetime problem; it does not establish a general driver defect.

With that lifetime guard, [three valid repeats](blue-engine-omarchy-20261112.json) exit normally: 1198/1194/1197 samples, medians 8.327/8.331/8.305 ms, configured surface 1474×1864. These are update-callback intervals, not displayed FPS. Raw reports and diagnostic core remain local; public reports omit private host/environment data. Report source revision is the pre-publication base plus the adapter changes in this commit; binary SHA identifies the tested executable.

Native input/focus, native screenshot capture and packaged installation are separate, unqualified gates. The headless screenshot path is not used to claim native visuals.
