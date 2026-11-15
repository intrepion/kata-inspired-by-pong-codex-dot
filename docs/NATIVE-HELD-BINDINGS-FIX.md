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

## Mac ARM64 follow-up

Source `cdf65cc`: all fourteen release tests pass (Miniquad 5, pixels 5, raw wgpu 4), and strict all-target release Clippy passes for all three. Existing release caches were reused, with one Cargo job; no new toolchain or cleanup. Each rebuilt native binary exits 0 with thirteen states through finished tick 1440 and six captures. Run times were 13.084s, 12.773s and 12.815s respectively; these include startup and concurrent checks, so are not performance measurements.

Tick-600 images were inspected for all three: both paddles, ball, divider and score marker render. Miniquad/pixels captures are 900×500; raw wgpu is 1800×1000. Pixel source-buffer evidence is not compositor proof. [Redacted binary identities and outcomes](evidence/native-held-bindings-mac-20261112.json). The input regression tests exercise direct event state; scripted replay does not prove actual Mac keyboard dispatch/focus handling. Historical packages and benchmark hashes remain unchanged and are not requalified by this follow-up. All test processes exited.

## Mac Pixels supported native gameplay follow-up

The corrected Pixels archive from the Mac package follow-up was extracted to a new UI-test folder and launched without scripted flags. Supported native keys/window observations verified Enter starts, held W moves up, held S moves down, and Space pauses/resumes. Actual window captures showed both paddles, ball and divider; this supplements the earlier CPU-source-buffer evidence. Score progression was visible in the native title.

The match reached 0–7 Over. Enter began a new match (already 0–1 Playing by the next observation); R followed promptly by Space showed exactly 0–0 Paused, confirming score reset. Application-menu Quit returned `procNotFound`, with no matching test process afterward. No physical overlapping-key release-order, OS-focus, resize, or clean-machine qualification is implied by this interaction batch. The archive/binary identities remain those in [corrected package evidence](packaging/mac-held-fixes-package-20261112.json); no rebuild or performance measurement was needed.

## Mac Pixels enlarged-window follow-up

The same corrected Pixels binary was launched normally and the exposed native window zoom action invoked. The resulting actual window capture was 2880×1558, with a centered 1800×1000 court and black margins. Both paddles, ball and divider remained coherent. Enter started play, held S moved the left paddle down, and Space paused at 0–1. A further resume/held-W/pause sequence moved that paddle upward while preserving the score. Thus gameplay and rendering continued in this enlarged state, without a new build.

A title-bar double-click did not visibly restore the smaller window, so the full resize/restore cycle is **not qualified** by this batch. The remaining AX tree did not expose the original window controls. This observation alone does not distinguish a window-manager/tool interaction from an application defect. Minimize, focus loss, DPI transitions and zero-size behavior remain unverified. App-menu Quit again returned `procNotFound`; a subsequent process check found no matching UI-test process. Screenshots were inspected directly, not saved as repository artifacts.
