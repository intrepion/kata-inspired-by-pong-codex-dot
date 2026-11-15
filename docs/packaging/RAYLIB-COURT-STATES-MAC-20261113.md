# Raylib court prompts: Mac native check (2026-11-13)

The raylib frontend source is Git blob `13ad96aa62f95156c3664782a280f5596b04f649` from `239e60afbc`, SHA-256 `ed303fb15e6e6ae03eea278c05d116763e0aed526c676a776815e5dfcf8ce021`. An offline locked cached release build succeeded with executable SHA-256 `a326745ac4828221985e0477efbbdf3686ab20addaaa2b9ddadea19dc1065e6c`; all 14 release tests passed, including state selection.

The native Mac window showed the complete centered “READY - SPACE OR ENTER TO START” prompt with its backing. Desktop automation sent Return and Space and then clicked the court and sent Return; the display remained Ready. Event delivery, focus, and application behavior were not independently distinguished, so this is not a confirmed application defect. The Paused and winner layouts were not visually verified in this run. App bundle and previous binary remain under `/tmp/pong-raylib-court-native-mac-20261113-v1/`.

This is a bounded native Ready visual check and adapter test result, not physical keyboard input, a match, or release/license qualification. Omarchy's separate scripted paused capture remains documented in `RAYLIB-COURT-STATES-OMARCHY.md`.
