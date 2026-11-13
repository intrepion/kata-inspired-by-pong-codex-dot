# Windows raw wgpu contact-timing recheck

On 2026-11-13, raw wgpu was built in Windows MSVC debug mode at `cc03133da5d2c4451edc11e8cd6ba31ef30f8963`, using the existing dependency cache. The executable SHA-256 was `7a3f2dda3f4b62d9e358f2c673bef8915effec7c4907f0011252f83b482e2612`.

The scripted replay exited 0 at tick 1440, with empty stderr, 13 [telemetry records](evidence/windows-wgpu-contact-timing-20261113.jsonl) and six app-owned PNGs. Discrete fields matched the contact-timing model reference and maximum absolute numeric difference was zero (allowance 0.001). The tick 480 paused source frame was inspected and shows the court, paddles and ball. This capture is app-owned output, not a desktop screenshot or proof of OS input.

Normal interactive input, physical keyboard, full match, focus/resize and packaging remain unverified on this build. The previous wgpu evidence remains historical.
