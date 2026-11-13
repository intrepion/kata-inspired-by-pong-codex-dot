# Low-level match-end restart instruction — macOS, 2026-11-13

The shared low-level renderer now shows R TO RESTART after a match, with a background backing that keeps the instruction legible against the court. It disappears during play. The renderer is used by Softbuffer, SDL3 and ash; no game input or scoring behavior changed.

Offline locked release-profile tests passed on Mac: 7 shared-renderer unit tests, 4 shared renderer integration tests, and 10 Softbuffer adapter tests. The added renderer regression checks the match-end backing and glyph pixels and their absence during play.

In the rebuilt native Softbuffer window, the instruction was absent during play. An unattended match ended CPU WINS at 3–7, with R TO RESTART legible beneath the winner. Supported desktop automation sent R: the game returned to Playing at 0–0 and the instruction cleared. Escape closed the window. This was automated input, not physical keyboard testing. SDL3 and ash native rendering, other window sizes, the player-win presentation and release qualification remain unverified.

The Softbuffer executable SHA-256 is `213c397251a4973e24b1a56fbc767f7b0b04c15e78db246507c09ac2874c97b2`; edited renderer SHA-256 is `aafe839a01c51cda9863400b1c9cf65b8eb500f2c9844f3a3c9aaa47e2078abf`. The isolated mixed tree used shared renderer base blob `dcb11dadcc350bb43655a760ced8c42d0128b3dd`, Softbuffer adapter blob `c46cb48675475fb145752446e58a740e8d78b3b0`, and core lib.rs blob `8a9a1f2ecdcbfefde4d67e3f4df0d5f1d2930dd3`, matching checkpoint `a5ed05de5da4e44b16aa51e7d79d565c7a5a7a6b`; other inputs were not all compared. Previous source and binary and the candidate app remain in `/tmp/pong-lowlevel-restart-hint-mac-20261113-v1`.

The prior [Mac Softbuffer license screen](SOFTBUFFER-MAC-LICENSE-SCREEN.md) is a manifest screen, not full clearance; system components, notices and clean-machine distribution checks remain open.
