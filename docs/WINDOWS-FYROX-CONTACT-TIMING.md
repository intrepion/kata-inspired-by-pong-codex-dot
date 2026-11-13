# Windows Fyrox contact-timing recheck

On 2026-11-13, Fyrox was built from `c18e9b832a0e5bc5b737fd836c53af327e0315a9` with locked offline dependencies in Windows MSVC debug mode, using a fresh target directory. Both frontend tests passed. Executable SHA-256: `6b311931c33ca3efe150162d0d4888598127540eb329ecbe26329ffbbb9730e0`.

The foreground scripted run exited 0 with empty stderr. Its 13 [telemetry records](evidence/windows-fyrox-contact-timing-20261113.jsonl) match the contact-timing model reference line-for-line through finished tick 1440; engine diagnostic lines were excluded. Six app-owned UI render-target PNGs were produced. The inspected tick 480 frame shows paused status, score 0:1, court, paddles and ball. These images are not desktop screenshots or evidence of OS input. The run generated a `data` directory in its isolated evidence directory, which was preserved.

Normal native interaction, physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Previous binaries and captures remain historical.

## Held-input pause recheck

At checkpoint `d239d7f8225a4eb9d82b5434e2113aa41661e21e`, including fix `1413206`, Windows MSVC offline locked debug test and build succeeded. All four tests passed: two benchmark argument tests and two adapter tests covering held movement across pause, repeat, release, fresh press, aliases and opposing bindings. The tested source blob was `c55c2b7f104eb66a5f179f29d2e5e58926ff6c0e`; fresh executable SHA-256 `266175e5e01f1e92885b0871e2daad1a4f4659e3994becb92a19d5252d9e308a`.

A separate scripted run exited 0 with empty stderr; its 13 telemetry lines matched `evidence/rust-core-contact-timing-model-20261112.jsonl` line-for-line through finished tick 1440. The new executable and raw logs are preserved in the task workspace `fyrox-held-pause-windows-20261113`; the earlier executable was copied there before the cached target was rebuilt and retained its recorded hash. These are adapter and scripted replay results, not evidence of OS key delivery, physical input, focus-loss delivery, graphical presentation, packaging or release qualification. Earlier captures remain historical.
