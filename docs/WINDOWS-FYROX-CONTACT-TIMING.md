# Windows Fyrox contact-timing recheck

On 2026-11-13, Fyrox was built from `c18e9b832a0e5bc5b737fd836c53af327e0315a9` with locked offline dependencies in Windows MSVC debug mode, using a fresh target directory. Both frontend tests passed. Executable SHA-256: `6b311931c33ca3efe150162d0d4888598127540eb329ecbe26329ffbbb9730e0`.

The foreground scripted run exited 0 with empty stderr. Its 13 [telemetry records](evidence/windows-fyrox-contact-timing-20261113.jsonl) match the contact-timing model reference line-for-line through finished tick 1440; engine diagnostic lines were excluded. Six app-owned UI render-target PNGs were produced. The inspected tick 480 frame shows paused status, score 0:1, court, paddles and ball. These images are not desktop screenshots or evidence of OS input. The run generated a `data` directory in its isolated evidence directory, which was preserved.

Normal native interaction, physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Previous binaries and captures remain historical.
