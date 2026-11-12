# Ratatui held-key correction, 2026-11-12

Enhanced terminals report press and release events. The adapter previously stored only one boolean per direction: holding Up and W, then releasing Up, incorrectly stopped movement while W remained held. Down/S had the same flaw. A `w` press followed by `W` release also left movement stuck.

Two new tests reproduced both failures against the previous code. The input state now tracks Up, W, Down and S independently, normalizing upper/lowercase movement characters. Opposite directions still cancel; releasing the last binding stops movement. The fallback terminal's existing latched-direction/X-stop behavior is retained; uppercase X is now also accepted.

## Omarchy verification

- Seven adapter tests pass (five existing, two regression tests).
- Strict Clippy for all adapter targets passes.
- Fresh optimized binary built; SHA-256 `2a2e63ef5abf7cd1ce2ea735983ca546e9d229cf81c7f2a4e1d68e5cdbd10c7b`.
- Scripted cell-export replay exits 0, reports 13 states through tick 1440, and writes six cell-buffer checkpoints. This mode is an in-process renderer/scenario test, not a native enhanced-terminal input test.

No desktop control, OS key injection, terminal screenshot or new package qualification is claimed. Existing archives and their hashes remain historical. Direct key-event regression tests establish the input correction; a real enhanced-terminal keyboard test remains separate.

## Windows follow-up

Source762b301: seven release tests and strict all-target release Clippy pass on Windows. Offline cached test/build used one Cargo job (3.29s/3.06s); no performance measurements or desktop-control retries during concurrent WSL work. Rebuilt executable SHA-256 `475d4c6f9547b60d893e7aa3ff810603ee87ae8090550b90ac9f157014cd9e1f`.

From a new isolated working directory, scripted TestBackend replay exited0 with thirteen records through finished tick1440 and six100x30 JSON cell buffers (3000cells each), plus matching text exports. These are deterministic cell-render/model checks, not real Windows enhanced-terminal input or screenshot evidence. The existing portable package keeps its earlier hash and has not been retroactively upgraded.

## Mac ARM64 follow-up

At source `762b301`, seven adapter tests pass and the debug binary rebuild succeeds using the existing target cache. SHA-256: `458c7da7cb03040618dc0afa52747f8cbc43e4877563a7d76833ed47a3867c96`. Scripted replay exits 0 with thirteen states through finished tick 1440 and six JSON/text cell-buffer pairs. The tick-600 text buffer was inspected: score/state, fallback control legend, court, paddles and ball are represented. This is cell evidence, not a screenshot.

A separate bounded native PTY run selected latched-input fallback, accepted Space to start (subsequent animated court updates), then lowercase q to quit. Process exit was 0 and exact `stty -g` settings before/after matched. This verifies basic PTY start/quit/restoration, not physical keyboard handling or the enhanced press/release terminal protocol; the latter correction is covered here by unit tests only. Historical package and release hashes remain unchanged.

## Action-key case consistency (2026-11-12)

A further source audit found that the terminal action dispatcher accepted lowercase `r`/`q`/Ctrl-`c` only, unlike case-insensitive movement. Shift or Caps Lock character events could therefore ignore restart/quit. A pure action-key normalizer is now used by the actual dispatcher; ASCII letters normalize while Escape/Space and all non-character codes are unchanged. Existing press-only action gating and Control-modifier requirement for Ctrl-C are unchanged.

The new regression failed on the original pass-through behavior at `Char('R') != Char('r')`, then passed after correction. Omarchy ARM64: all eight optimized tests and strict all-target Clippy pass. The fresh optimized binary completes the bounded scripted cell replay with 13 states through tick 1440, finished=true, and six JSON/text cell checkpoints. This is model/dispatcher and in-process terminal-buffer evidence, not an actual terminal keypress or screenshot. Prior package hashes are historical; no new package, timing or physical-input result is inferred.

### Mac action-key case follow-up

At source `78d9524`, all eight adapter tests pass in the debug profile. An offline cached rebuild took 0.39s after tests; binary SHA-256 `553c0b795ae6bd3b69ea0e39cc74df8316ef1d8edf3ea8affdc1ccf16204d456`. No new dependency cache or optimized build was needed.

A native PTY run selected the latched-input fallback. Space started play and animated cell updates appeared. Sending uppercase R followed by Space reset the visible player score to zero, recentered the ball and showed Paused. Uppercase Q then exited successfully: `PONG_EXIT=0`, with exact before/after terminal settings matching (`RESTORE=MATCH`). This tests real terminal-byte dispatch and cleanup rather than only calling the normalizer. It does not establish physical Shift/Caps Lock behavior, enhanced-terminal key release events, a GUI screenshot, or package/release qualification. Historical binary/package hashes remain unchanged in earlier evidence.
