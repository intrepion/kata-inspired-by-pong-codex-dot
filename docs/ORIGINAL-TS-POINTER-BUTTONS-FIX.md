# Original TypeScript pointer-button ownership fix

2026-11-12, base `eb6e2e7`. The original browser app's Up/Down buttons shared one numeric touch direction. A second finger overwrote the first, and any pointer-up/cancel/lost-capture event cleared every held button. This could stop movement while another finger remained held; opposite buttons also selected the last press instead of cancelling.

The original TypeScript source now tracks captured button pointer IDs independently. Releasing/cancelling one pointer removes only that pointer. Multiple fingers on the same direction do not increase speed, opposing directions cancel, and window blur clears all pointer ownership. Existing keyboard state and canvas-drag behavior are unchanged. `game.js` was regenerated with the pinned TypeScript compiler, not hand-edited.

## Verified on Omarchy ARM64

Two focused tests failed with the prior shared-direction behavior (`0 != -1` after releasing one of two Up pointers, and `1 != 0` with opposite directions). Both pass after correction. The strict TypeScript build and all 17 original game/controller tests pass, including physics, timing, scenario and pointer regressions. No dependency or system-setting changes.

This is controller and compiled-handler evidence. Actual browser multi-touch delivery, capture cancellation and mixed keyboard/touch behavior still require browser/device qualification; no real-touch or screenshot pass is claimed. Existing release/package/timing evidence is not replaced by this change.

## Windows follow-up

On 2026-11-12, an isolated copy of the original source at `c37e64c` compiled with the existing TypeScript 5.9.3 installation and passed all 17 Node tests, including both pointer ownership regressions. The compiled `game.js` matches the checkpoint blob. This is model/controller coverage, not native input evidence.

The same checked-in JavaScript was served from `127.0.0.1` and opened in Chrome using supported browser controls. Start changed the page to an active match; Pause showed “Take a breather” and “Paused”. This is a basic browser smoke check, not a multi-pointer test. Simultaneous touch contacts, pointer capture cancellation, mixed keyboard/touch, and physical-device behavior remain unverified.
