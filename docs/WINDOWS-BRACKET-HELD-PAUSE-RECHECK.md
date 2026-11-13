# Windows bracket-lib held-input pause recheck

On 2026-11-13, bracket-lib was rebuilt from `e5d4ce01f8201abb813a28c677b8b20cf6e90103`, including held-input pause fix `5fb4cc675ca039466422fe10288efe98375a7835`, with locked offline Windows MSVC debug dependencies. Executable SHA-256: `f9b9da39af50655e7610f7963de0598bbd7e1c849942f893db31d4066a4ce62d`. All 11 frontend/action and rendering tests passed, including pause suppression, key repeats, independent aliases and focus reset. These are mapped event/model tests, not OS or physical-key delivery evidence. The prior executable was copied to the local evidence directory before rebuilding and retains its earlier hash.

The foreground scripted replay exited 0 with empty stderr, six app-owned PNGs and 13 [telemetry records](evidence/windows-bracket-held-pause-20261113.jsonl) matching the contact-timing reference line-for-line. The inspected tick 480 frame shows paused status, score, court, paddles and ball. This application-rendered image is not a desktop screenshot; scripted replay bypasses real held-key input and does not independently verify the pause fix in an OS event stream.

Native held input across pause, physical input, focus/resize, full match, packaging and release qualification remain open for this binary. Previous and new local artifacts remain preserved. No desktop approval was retried.
