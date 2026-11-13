# Windows Ratatui contact-timing recheck

On 2026-11-13, Ratatui was built from `c8caa9b8c17b296b1401da88f91786db364b3d56` with locked offline dependencies in Windows MSVC debug mode, in a fresh target directory. All eight frontend tests passed. Executable SHA-256: `c2a6a19a4df58e49914b715c6061ff42fbac2f83d5f0532c82f1d3d2c01f527b`.

The scripted TestBackend run exited 0 with empty stderr. Its 13 [telemetry records](evidence/windows-ratatui-contact-timing-20261113.jsonl) match the contact-timing model reference line-for-line through finished tick 1440. Six app-owned cell JSON/text pairs were produced. The inspected tick 480 text contains the paused status, score 0:1, fallback controls and court. These generated cells are not a terminal screenshot or proof of OS input.

The prior Windows Terminal access limitation described in [the frontend notes](../experiments/ratatui/README.md) was not retested. Real terminal presentation and interaction, physical input, resize/focus and terminal restoration, full match, packaging and release qualification remain open for this revision. Earlier artifacts remain historical.

## Enhanced-key repeat recheck

At checkpoint `632425e6788a4083c5d399eed062a4f4d7e8a9ee`, including fix `10b0aba`, the offline locked Windows MSVC debug build and all ten tests passed, including enhanced repeat suppression after pause/reset, independent aliases and existing fallback and renderer tests. The tested source blob was `d93e737fab26b749f7f20dc6898b959a19b63f32`; executable SHA-256 `d28983ba22e04a6912db026d7ab28bcd80f4cab8e8801692e0a70c889c087f8f`. Cargo warned that incremental-cache hard linking failed and it copied files instead; compilation and tests completed.

The TestBackend scripted run exited 0 with empty stderr and all 13 telemetry lines equal to the model reference through finished tick 1440. Six text/JSON terminal-cell pairs were exported; the tick 480 text shows 0:1 and Paused. The binary, previous binary, raw logs and cell exports remain in task workspace `ratatui-held-repeat-windows-20261113`. These are adapter/model and app-owned cell-render checks, not terminal protocol, native keyboard, physical input, screenshot, package or release qualification. The prior package and captures remain historical.
