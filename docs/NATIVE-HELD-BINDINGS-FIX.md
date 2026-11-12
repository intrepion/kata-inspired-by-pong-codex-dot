# Miniquad, pixels and raw wgpu held-binding fixes

The three adapters stored one boolean per movement direction. Releasing Up after pressing Up and W cleared upward movement even though W remained held. The same defect affected Down/S and either release order.

Each adapter now has a small engine-key-specific input boundary tracking four bindings independently. Repeated press events are idempotent, opposite directions cancel, and existing focus/minimize clearing resets all bindings. Rendering, scenario timing, game physics, start/pause actions and benchmark protocols are unchanged.

## Omarchy ARM64 verification, 2026-11-12

| Adapter | Tests | Strict all-target Clippy | Fresh optimized native replay |
| --- | --- | --- | --- |
| Miniquad | 5 pass | pass | exit 0; 13 states; six app-owned PNGs |
| pixels | 5 pass | pass | exit 0; 13 states; six CPU-source PNGs |
| raw wgpu | 4 pass | pass | exit 0; 13 states; six app-owned PNGs |

Each adapter includes tests for both directions/both release orders, repeated press, opposite-direction cancellation and clearing. As a regression-sensitivity check, temporarily restoring the old shared-direction overwrite semantics at the extracted input boundary makes each alias test fail (exit 101). Restoring the corrected code returns all suites to green. That mutation was never published or used to build the tested release binaries.

All native replays finish at tick 1440. Tick 600 captures were inspected: court, paddles, ball and existing score markers render. These are application-owned evidence, not compositor screenshots; pixels exports its CPU source buffer rather than the presented window. The scripted scenario bypasses real key events, so direct input tests establish the fix while native replay checks integration did not regress.

Binary hashes are recorded in [the redacted result](evidence/native-held-bindings-linux-20261112.json). Prior package and timing hashes remain historical; this batch does not requalify packages, benchmark performance, Windows/Mac binaries or actual OS-keyboard behavior.

## Windows x64 GNU follow-up

At sourcecdf65cc, all14release tests pass (Miniquad5/pixels5/wgpu4). Offline cached builds used one Cargo job and completed in1.64s/5.21s/5.74s. Native scripted runs from new isolated directories each exit0, emit thirteen state records ending finished tick1440, and produce six app-owned PNGs. Tick600 inspected for each: paddles, ball, divider and existing tally-style score marks visible. pixels remains CPU-source-buffer evidence, not final presentation.

[Redacted Windows hashes/results](evidence/native-held-bindings-windows-20261112.json). Tests exercise input boundaries; scripted replay bypasses real key events. No OS-input/focus, new benchmark, strict-Clippy rerun or updated portable-package qualification is claimed. Previous timing/archive hashes remain historical. No desktop approval retry or large parallel build was used during concurrent WSL CI work.
