# Godot Mac contact-timing attempt

macOS ARM64 source `2582b78af9ffecb254c4367184cad76640999e31`, including physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. An offline locked debug GDExtension build reused a copy-on-write cache; a handful of small dependencies and local Pong crates compiled in 10.49 seconds. Extension SHA-256 `cbd5fa8f5c7d848fa68187bf5bb77bc508c01c80b645affec8e1b80e9a3d82b6`.

A separate project copy and extension-list file were created for a sandboxed Godot 4.5.1 headless smoke run. It failed to open `user://logs` and crashed in `RotatedFileLogger` before assertions (exit -6). This is an environment-limited failed attempt, not a passing integration or gameplay test; no editor-shutdown conclusion follows. Project and build artifacts are preserved. A suitable writable user-data context and current-physics runtime checks remain open.

## Writable-log follow-up

Godot 4.5.1's supported `--log-file` option was pointed to an absolute path under `/tmp`, with the same isolated project and extension. Headless `--script res://smoke.gd` exited 0 and printed `GODOT_PONG_SMOKE_PASS` for extension, start, movement, pause, resume and reset. A separate headless `-- --scripted-test` run also exited 0, producing [13 telemetry records](evidence/macos-godot-contact-timing-20261113.jsonl) exactly equal to the current contact-timing model reference, finishing at tick 1440. Both runs emitted a macOS system CA certificate retrieval error; the smoke and replay still completed. The initial failed attempt above is retained.

These are headless integration and scripted-model-driven scene checks; no graphical viewport capture, native or physical input, editor shutdown, packaging or release qualification is implied. The explicit log path resolves this test's log setup failure; it does not establish a general default user-data fix.

## Native window check

The same isolated project and extension were launched through a locally assembled pilot wrapper with an explicit writable log path. App selection by wrapper ID reported that the application was not found, but desktop inventory showed the Godot engine running, and selecting its existing process exposed the `Pong Godot + Rust (DEBUG)` window. Supported desktop automation pressed Space to start, held W to move upward, paused with Space, resumed, held S, reset with R and paused again. The court and `Paused` status were visible at 0–0 with a centered paddle after reset; Escape exited the engine. These are bounded automated native interactions, not physical input, a complete match or a distribution package. The pause state appeared in the status line; the in-court pause message is not drawn by the current Ready/Over-only condition. Earlier headless results and limitations remain as above.

## Pause-prompt fix

The in-court pause text was unreachable because the rendering condition allowed only Ready and Over. The Godot frontend now includes Paused and draws a court-colored backing behind the status message so the ball cannot obscure it. Tested GDScript blob `8f216e4c3587fa5b1805a578ae42777dc47ef5ad`, based on checkpoint `7d4b5ba919bc9d2e64a7ab1553dc1bfd8f52476d`; the same debug extension and isolated project above were used.

With the writable `--log-file` option, the headless smoke exited 0 and passed extension/start/move/pause/resume/reset assertions; the known macOS certificate-store error was emitted. In the native Godot window, desktop automation started and paused the game, visibly inspected the unobscured `Paused - Space to resume` prompt, then resumed and confirmed it disappeared. Escape exited the app. This is automated desktop interaction, not physical input or distribution qualification; prior editor-shutdown and other-platform gaps remain.

## Full-match desktop check

A separate run of the same isolated project, GDScript blob and debug extension above used supported desktop automation to press Space, then left the player paddle unattended. The visible score reached **3–7**, the state showed Over, and the court displayed `Match over - Space to restart`. Space restarted at **0–0, Playing**; Escape exited the engine. This is an automated native interaction, not physical input or a distribution package. The headless scripted replay remains separate evidence.
