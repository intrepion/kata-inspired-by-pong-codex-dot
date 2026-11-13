# Godot Mac contact-timing attempt

macOS ARM64 source `2582b78af9ffecb254c4367184cad76640999e31`, including physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. An offline locked debug GDExtension build reused a copy-on-write cache; a handful of small dependencies and local Pong crates compiled in 10.49 seconds. Extension SHA-256 `cbd5fa8f5c7d848fa68187bf5bb77bc508c01c80b645affec8e1b80e9a3d82b6`.

A separate project copy and extension-list file were created for a sandboxed Godot 4.5.1 headless smoke run. It failed to open `user://logs` and crashed in `RotatedFileLogger` before assertions (exit -6). This is an environment-limited failed attempt, not a passing integration or gameplay test; no editor-shutdown conclusion follows. Project and build artifacts are preserved. A suitable writable user-data context and current-physics runtime checks remain open.

## Writable-log follow-up

Godot 4.5.1's supported `--log-file` option was pointed to an absolute path under `/tmp`, with the same isolated project and extension. Headless `--script res://smoke.gd` exited 0 and printed `GODOT_PONG_SMOKE_PASS` for extension, start, movement, pause, resume and reset. A separate headless `-- --scripted-test` run also exited 0, producing [13 telemetry records](evidence/macos-godot-contact-timing-20261113.jsonl) exactly equal to the current contact-timing model reference, finishing at tick 1440. Both runs emitted a macOS system CA certificate retrieval error; the smoke and replay still completed. The initial failed attempt above is retained.

These are headless integration and scripted-model-driven scene checks; no graphical viewport capture, native or physical input, editor shutdown, packaging or release qualification is implied. The explicit log path resolves this test's log setup failure; it does not establish a general default user-data fix.

## Native window check

The same isolated project and extension were launched through a locally assembled pilot wrapper with an explicit writable log path. App selection by wrapper ID reported that the application was not found, but desktop inventory showed the Godot engine running, and selecting its existing process exposed the `Pong Godot + Rust (DEBUG)` window. Supported desktop automation pressed Space to start, held W to move upward, paused with Space, resumed, held S, reset with R and paused again. The court and `Paused` status were visible at 0–0 with a centered paddle after reset; Escape exited the engine. These are bounded automated native interactions, not physical input, a complete match or a distribution package. The pause state appeared in the status line; the in-court pause message is not drawn by the current Ready/Over-only condition. Earlier headless results and limitations remain as above.
