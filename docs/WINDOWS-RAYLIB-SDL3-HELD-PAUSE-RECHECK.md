# Windows raylib and SDL3 held-input pause recheck

On 2026-11-13, raylib-rs and SDL3 were rebuilt from `a27ea33ff9fa7450f2e603ada0e6cce3b681fb1b`, including pause fixes `5af5e9a` and `e862cc4`, with locked offline Windows MSVC debug dependencies. Raylib passed all five polled-input/frontend tests and SDL3 passed all four. These include held-direction suppression until release and alias behavior; they test the input gates, not OS or physical-key delivery. SDL3 samples state after each event batch, so a release and repress entirely between polls cannot be distinguished from a continuous hold.

Raylib executable SHA-256: `ba42e6c923376f394e74d8f4fbb809afd3d2a13e38903b9b977c1cfd3dd8ecd1`; SDL3 executable SHA-256: `2d02caad6708ea8df33c225d3395eebe3038b9bd5b1a3592bbefd197bdb63874`. Both ordinary Cargo debug builds and tests repeated the documented `LNK4098` conflicting-runtime warning, plus an incremental-cache hard-link fallback; neither was suppressed. Previous binaries were copied to the local evidence directory before rebuild.

Each foreground scripted run exited 0 with empty stderr, six app-owned PNGs and 13 records matching the contact-timing reference line-for-line: [raylib](evidence/windows-raylib-held-pause-20261113.jsonl), [SDL3](evidence/windows-sdl3-held-pause-20261113.jsonl). The inspected tick 480 frames show score, court, paddles, ball and their pause indicators. These are application-rendered outputs, not desktop screenshots. Scripted replay bypasses real held-key input and does not prove the fixes in an OS event stream.

Native held input, physical input, focus-change polling, full match, packaging, runtime-warning resolution and release qualification remain open. Previous and new local artifacts remain preserved. No desktop approval was retried.
