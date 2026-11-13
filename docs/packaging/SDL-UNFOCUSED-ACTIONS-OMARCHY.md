# SDL3 unfocused actions — Omarchy, 2026-11-13

After processing a focus-loss event, SDL3 previously still accepted a queued Space or R keydown: Space could resume a paused game, and R could start a new match while unfocused. The event loop now tracks window input focus, pauses on focus loss, and ignores live pause/restart actions until focus is regained. Its initial focus state comes from SDL's window input-focus flag. Scripted and benchmark modes retain their existing separate paths.

A regression failed before the change and passed afterward: after focus loss, Space and R leave the game paused and preserve scores; Space resumes when focus is restored. All 11 locked offline SDL3 tests passed and a debug build passed. Source Git blob `42c616548220022e4ba0b183c06ad75ce61ea160`; debug binary SHA-256 `a9be451fd8fe6549749b4d2901e2894ea36d1b0808a37d21d17f51e597c9fd58`.

The regression invokes the same live-action boundary used by the event loop; actual OS focus delivery, queued physical key events, rendered replay, clean-machine packaging and other platforms were not tested. Existing artifacts were retained.
