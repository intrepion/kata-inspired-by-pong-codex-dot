# Ratatui held-key correction, 2026-11-12

Enhanced terminals report press and release events. The adapter previously stored only one boolean per direction: holding Up and W, then releasing Up, incorrectly stopped movement while W remained held. Down/S had the same flaw. A `w` press followed by `W` release also left movement stuck.

Two new tests reproduced both failures against the previous code. The input state now tracks Up, W, Down and S independently, normalizing upper/lowercase movement characters. Opposite directions still cancel; releasing the last binding stops movement. The fallback terminal's existing latched-direction/X-stop behavior is retained; uppercase X is now also accepted.

## Omarchy verification

- Seven adapter tests pass (five existing, two regression tests).
- Strict Clippy for all adapter targets passes.
- Fresh optimized binary built; SHA-256 `2a2e63ef5abf7cd1ce2ea735983ca546e9d229cf81c7f2a4e1d68e5cdbd10c7b`.
- Scripted cell-export replay exits 0, reports 13 states through tick 1440, and writes six cell-buffer checkpoints. This mode is an in-process renderer/scenario test, not a native enhanced-terminal input test.

No desktop control, OS key injection, terminal screenshot or new package qualification is claimed. Existing archives and their hashes remain historical. Direct key-event regression tests establish the input correction; a real enhanced-terminal keyboard test remains separate.

## Mac ARM64 follow-up

At source `762b301`, seven adapter tests pass and the debug binary rebuild succeeds using the existing target cache. SHA-256: `458c7da7cb03040618dc0afa52747f8cbc43e4877563a7d76833ed47a3867c96`. Scripted replay exits 0 with thirteen states through finished tick 1440 and six JSON/text cell-buffer pairs. The tick-600 text buffer was inspected: score/state, fallback control legend, court, paddles and ball are represented. This is cell evidence, not a screenshot.

A separate bounded native PTY run selected latched-input fallback, accepted Space to start (subsequent animated court updates), then lowercase q to quit. Process exit was 0 and exact `stty -g` settings before/after matched. This verifies basic PTY start/quit/restoration, not physical keyboard handling or the enhanced press/release terminal protocol; the latter correction is covered here by unit tests only. Historical package and release hashes remain unchanged.
