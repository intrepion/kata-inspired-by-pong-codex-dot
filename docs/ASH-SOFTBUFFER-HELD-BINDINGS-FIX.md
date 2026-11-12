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
