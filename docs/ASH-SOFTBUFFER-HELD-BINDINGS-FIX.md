# Ash and softbuffer held-binding correction

## Defect and correction

Both adapters assigned one direction boolean directly from each W/Up or S/Down event. Holding two aliases and releasing either one incorrectly stopped movement while the other remained held. The correction tracks four physical bindings independently and derives each direction from its two aliases. Focus loss clears both the binding storage and common application input. Physics, rendering and the benchmark protocol are unchanged.

Before correction, the new release-order regression failed in each adapter against the extracted old behavior. After correction, the tests cover both directions and both release orders, repeat idempotence, opposite directions, and focus clearing without stale-held keys.

## Windows validation, 2026-11-12

- Ash: all six release tests pass; softbuffer: all four release tests pass.
- Both strict offline release Clippy checks pass with all targets and warnings denied.
- Both freshly rebuilt native scripted runs exit 0, emit thirteen state records and finish at tick 1440.
- Each emits six app-owned PNG captures. Tick 600 from each was inspected: court, paddles, ball and 0–1 score are coherent.
- Binary hashes and runtime counts are in [Windows evidence](evidence/ash-softbuffer-held-bindings-windows-20261112.json).

These are input-handler unit regressions plus native scripted integration. Scripted inputs do not prove physical OS keyboard delivery or real focus-event handling, and app-owned captures are not desktop screenshots. Previous benchmark and portable-package hashes remain historical; no new timing or package qualification is claimed. Mac and Linux requalification remains separate.

## Mac ARM64 follow-up

At source `af1db13`, all six ash and four softbuffer release tests pass. Strict release Clippy passes for all targets in both adapters; upstream `block 0.1.6` future-incompatibility warning remains. Ash unit tests do not initialize Vulkan. Its previously documented Mac loader/MoltenVK runtime blocker remains; no ash runtime or package pass is claimed and no loader was installed.

Softbuffer was rebuilt, bundled and extracted into a fresh directory. Extracted binary hash matched the manifest; native replay exited 0 in 12.753 seconds with thirteen states through finished tick 1440 and six captures. Tick600 at1800×1000 was inspected: 0–1 score, both paddles, ball and divider coherent. Three package safety guards pass. Removing only the extracted app preserved the adjacent sentinel; remaining files are exactly the pilot notice and sentinel. [Redacted Mac package/run evidence](packaging/mac-softbuffer-held-package-20261112.json).

These checks establish input-model regressions and extracted runtime integration, not physical keyboard/focus behavior, new performance results, signing, clean-machine installation or redistribution clearance. Earlier binary/package/timing hashes remain historical. Processes exited.

## Omarchy ARM64 requalification

At `af1db13`, fresh optimized builds, all ten adapter tests (ash six, softbuffer four) and strict all-target Clippy pass. Both rebuilt native executables exit 0 after 13 scripted states through tick 1440 and six app-owned captures each. Tick 600 inspections show score 0:1, both paddles, ball and divider. Softbuffer replay began after its fresh build completed.

[Redacted hashes/results](evidence/ash-softbuffer-held-bindings-linux-20261112.json). These app-owned images are not compositor evidence or physical-key tests. Existing benchmark and package hashes remain historical; no new timing or package qualification is inferred. Raw logs and captures remain local, and both native processes exited.
