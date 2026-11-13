# Fyrox keypad Enter — macOS, 2026-11-13

Fyrox now accepts `NumpadEnter` for the existing Enter start, resume and match-end restart action. Active-play Enter remains a no-op, R remains active restart, and match-end restart suppresses held movement for both Enter bindings.

All 10 offline locked release-profile tests passed on Mac. The added synthetic key-code regression checks Ready, unfocused, repeat, release, active-play and pause/resume behavior; the existing match-end movement test now also covers keypad Enter. The rebuilt native window changed from Ready to Playing after the desktop tool sent `KP_Enter`, paused with Space, then returned to Playing after another `KP_Enter`. Score progressed after resume. These were automated commands, whose mapping to physical keypad events was not independently observed.

The candidate executable SHA-256 is `34a4be5cfc29e81a990046719a85f51f3f4ebd3abbb6f2c0b45726cddedcd54b`; edited adapter SHA-256 is `a615bc2e2c5bdc42a929cc9344efffba5f67e64fc58d16b795309b8b480a5474`. The isolated mixed tree used adapter base blob `82d2c4c317a96c76e6e396aad624876c543f833c` from checkpoint `e17a16cf9bd4c9a55784804d799e86b3e15e8b0c`; other inputs were not all compared. Previous source and executable and the candidate app remain in `/tmp/pong-fyrox-keypad-mac-20261113-v1`.

Physical keypad delivery, native match-end restart, other platforms and release qualification remain unverified. The prior [Mac Fyrox MPL screen](FYROX-MAC-MPL-SCREEN.md) identifies restricted dependency paths; this gameplay change does not clear that license blocker or authorize distribution.
