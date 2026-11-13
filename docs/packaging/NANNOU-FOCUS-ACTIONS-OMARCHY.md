# Nannou unfocused action gate — Omarchy, 2026-11-13

Nannou paused on an unfocused callback but continued processing polled action keys. The adapter now tracks the paired window focus callbacks and skips live polling and accumulated simulation time while unfocused. A focus-gain callback reopens the gate; the previous focus-loss pause and movement suppression remain. Scripted and benchmark paths remain separate. The initial focus assumption remains true until a callback arrives.

A polling-boundary regression failed before the gate and passed afterward: unfocused restart, pause/resume and Escape do not act, while focused resume and Escape remain accepted. All 14 locked offline debug tests passed. A fresh debug build succeeded (SHA-256 `9b23c182ef4fdff561af11327f867f0fd31eb52b25419930fa5344f816d419f7`); tested source blob `668ff59e2355b7e3e22a24689aef153bdaa82ead` was based on checkpoint `309a6ea5ac5589312017ed3ed19f1c9bc5618f26`. The previous executable was copied to `/tmp/pong-nannou-focus-gate-20261113-a/preexisting-debug-binary` before rebuilding.

With host-display access, the app-owned scripted replay exited 0 and 13 telemetry records byte-matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Renderer information appeared on stderr; logs remain at `/tmp/pong-nannou-focus-gate-20261113-a`.

The regression supplies synthetic polled values; neither the replay nor the test proves native callback delivery, physical input, or what happens if an action key is reported as newly pressed on the same frame as focus gain. Those cases, other platforms, full gameplay, installation and release qualification remain open. Prior evidence is retained.
