# Windows Pixels and raw wgpu explicit-pause recheck

On 2026-11-13, Pixels and raw wgpu were rebuilt from `679e0040688ff388d9b26628100dde09016870d6`, including shared pause-fix commit `799cd5e33c059d72d6f2ab035d02a64fa115f324`, with locked offline Windows MSVC debug dependencies. Pixels passed all seven frontend/action tests; raw wgpu passed all six. These tests include W, Up, S and Down clearing on explicit pause, ignoring repeat and allowing a fresh press. They exercise mapped action logic, not OS or physical-key delivery. Pixels' build reported an incremental-cache hard-link fallback but succeeded.

Pixels executable SHA-256: `4f8ce4cc24ffa56ed4d83c963226fc8b92b515a54aa54513e961ce0185e800dc`; raw wgpu executable SHA-256: `a6becf11c6e4fbffd3aba0da4ec6d8e6ad2df9af986d6c6a9d587d90fc624e16`. The previous Pixels executable was copied to the local evidence directory before rebuild; the raw wgpu build used a fresh target directory, preserving earlier artifacts.

Each foreground scripted run exited 0 with empty stderr, six app-owned PNGs and 13 records matching the contact-timing reference line-for-line: [Pixels](evidence/windows-pixels-pause-20261113.jsonl), [wgpu](evidence/windows-wgpu-pause-20261113.jsonl). The inspected tick 480 frames show courts, paddles and balls. Pixels captures its application source buffer; raw wgpu captures an application surface readback. Neither is a desktop screenshot. Scripted replay bypasses real held-key input and does not prove the pause fix in an OS event stream.

Native interaction, physical input, full match, focus/resize, packaging and release qualification remain open for these binaries. Prior outputs and new local evidence remain preserved in their distinct directories. No previous desktop-approval timeout was retried.
