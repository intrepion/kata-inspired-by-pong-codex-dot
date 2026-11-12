# Shared GUI held-binding regression, 2026-11-12

## Defect and correction

The GTK, Slint and GPUI adapters share `pong-gui-support::Model`. The model previously set a direction flag directly from the latest key event. Holding W and Up together, then releasing W, stopped upward movement even though Up was still held. The equivalent S/Down combination also failed.

Movement flags are now derived from all currently held bindings after each key transition. Action-key repeat suppression, opposite-direction cancellation, focus-loss clearing, the fixed-step clock, and game physics are unchanged.

## Verification on Omarchy ARM64

- Both new regression tests failed against the old implementation (four existing tests passed).
- After the correction, all six shared GUI model tests pass. The upward test also advances the game and verifies actual paddle displacement while the remaining binding stays held.
- Strict Clippy passes for the shared model and all its test targets.
- Seventeen core/scenario tests pass, including portable snapshots and seeded replay.
- A fresh optimized GTK build incorporating this model exits normally during its native scripted replay: 13 records, final tick 1440, `finished: true`.
- GTK binary SHA-256: `acfe559430ed7742751bbe00084994cb08deec94ac872f60903559e0c0135b23`.

The regression tests exercise the model's key-event boundary directly. The scripted runtime replay bypasses real keyboard input and is only an integration non-regression check. This is **not** a new OS-keyboard, focus, compositor, GPUI rendering or cross-platform runtime pass. Slint and GPUI consume this same corrected source when rebuilt; their existing binary/package results describe their previous hashes and are not retroactively requalified.

The planned GPUI package review was deferred to fix this concrete gameplay issue first. No package, public release, system setting or desktop-control permission changed in this batch.

## Mac ARM64 model follow-up

All six shared model tests, including both overlapping held-binding regressions, pass on Mac after the correction. Strict Clippy passes with all targets. This is a source-level Mac check only; existing GTK/Slint/GPUI binaries and package hashes have not been requalified, and no new OS keyboard/focus pass is claimed.
