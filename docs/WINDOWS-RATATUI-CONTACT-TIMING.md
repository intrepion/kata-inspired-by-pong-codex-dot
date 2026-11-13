# Windows Ratatui contact-timing recheck

On 2026-11-13, Ratatui was built from `c8caa9b8c17b296b1401da88f91786db364b3d56` with locked offline dependencies in Windows MSVC debug mode, in a fresh target directory. All eight frontend tests passed. Executable SHA-256: `c2a6a19a4df58e49914b715c6061ff42fbac2f83d5f0532c82f1d3d2c01f527b`.

The scripted TestBackend run exited 0 with empty stderr. Its 13 [telemetry records](evidence/windows-ratatui-contact-timing-20261113.jsonl) match the contact-timing model reference line-for-line through finished tick 1440. Six app-owned cell JSON/text pairs were produced. The inspected tick 480 text contains the paused status, score 0:1, fallback controls and court. These generated cells are not a terminal screenshot or proof of OS input.

The prior Windows Terminal access limitation described in [the frontend notes](../experiments/ratatui/README.md) was not retested. Real terminal presentation and interaction, physical input, resize/focus and terminal restoration, full match, packaging and release qualification remain open for this revision. Earlier artifacts remain historical.
