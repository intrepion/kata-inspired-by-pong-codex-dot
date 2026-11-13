# Windows Miniquad contact-timing recheck

On 2026-11-13, Miniquad was built in Windows MSVC debug mode from `663aa403e04a571cb1b43d9731fcb83090630436`. The executable SHA-256 was `c116675be0ded5680fc3302a4af2c874ffc8c6901a6c0b2aff0396064e4a70cc`. All seven adapter/model tests passed; mapped action tests do not prove operating-system key delivery.

The fresh scripted replay exited 0 at tick 1440 with empty stderr, 13 [telemetry records](evidence/windows-miniquad-contact-timing-20261113.jsonl), and six app-owned PNGs. All discrete fields matched the revised contact-timing model reference; maximum absolute numeric difference was zero (allowance 0.001). The paused tick 480 source capture was inspected and shows the court, paddles and ball. This is not a desktop screenshot or real-input test.

No new normal interactive test was performed. The prior [Windows key-delivery diagnosis](MINIQUAD-WINDOWS-INPUT-DIAGNOSIS.md) recorded supported automation keys arriving as Unknown; this replay and mapped action tests do not resolve it. Physical keyboard, full match, focus/resize and packaging remain unverified for this revision. Previous evidence and binaries remain historical.
