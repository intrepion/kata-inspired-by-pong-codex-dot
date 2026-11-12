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

Windows follow-up: fresh Slint release and all six shared GUI support release tests passed, including both simultaneous-binding regressions. Extracted Slint scripted replay and six own-window snapshots passed; this does not claim real Windows keyboard dispatch/focus acceptance. See [Slint Windows pilot](packaging/SLINT-WINDOWS-PILOT.md).

## Modifier-case identity follow-up

A second model-boundary regression was reproduced: pressing `w` and releasing `W` (or the reverse; also S/s) left the paddle moving, because the release did not remove the differently cased held-set entry. Repeated `R` after held `r` also incorrectly restarted the game and reset scores again. Both new tests failed before this correction.

The model now normalizes supported key identity before held-state/repeat handling. W and Up remain distinct bindings, so the previous overlapping-key fix is retained. Eight GUI-model tests and strict all-target Clippy pass on Omarchy ARM64. Tests check paddle position after release and score preservation during case-changed repeat, not only boolean flags.

This is direct key-event/model evidence. No claim is made that a particular OS/toolkit generated this exact sequence during native testing, nor that old Slint/GTK/GPUI binaries were rebuilt or requalified by these unit tests. Case-insensitive logical handling is intentionally limited to the documented controls.

## Windows modifier-case correction follow-up

All eight GUI support release tests pass on Windows, including case-changing release/repeat and the earlier overlapping bindings. Both GPUI and Slint rebuilt against the correction at cc9f0ef (shared source from20e1ac8). Bounded replays from new isolated working directories exited0 with thirteen states through finished tick1440: GPUI13.138s, Slint12.721s. Slint produced six own-window snapshots; tick600 inspected, score/controls/court and attribution intact. GPUI remains state-only. No desktop-control retry or OS-input claim.

New binary SHA-256: GPUI `738e15317b4d88629c4bdc4b6c14bee45671e4fbf6bc95badd68cc6996198b9b`; Slint `a5c965051b024ee9d1300f474378ca35db1e0a7c4b5fdbb6894e60ffca70a641`. Existing package archives retain earlier hashes and are not retroactively upgraded by these rebuilt-runtime checks.

## Mac Slint rebuild after both corrections

At source `5af98ee`, all eight shared-model tests pass on Mac, including overlapping direction bindings and Shift/case-change release/repeat regressions. The pinned Slint 1.18.1 native debug adapter was rebuilt against this source using its existing target directory (65 seconds; dependencies required recompilation). Native scripted replay exited 0 in 17.326 seconds, thirteen states through finished tick 1440, six own-window PNGs. [Tick 600](evidence/slint-keyfix/mac-tick-0600.png), 1920×1300, shows readable score/control text, court, both paddles, ball and the required AboutSlint display. [Binary identity and run outcome](evidence/slint-keyfix/mac-result.json).

This requalifies native scripted integration and the inspected layout after both model fixes, not actual OS keyboard/focus behavior. The replay bypasses key callbacks; unit regressions cover those model transitions. This is a debug build, not a release timing sample or packaged installer. The old bundled Slint application remains unchanged; no new public binary, signing or licensing clearance is implied.
