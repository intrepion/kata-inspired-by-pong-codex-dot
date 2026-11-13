# Windows raylib contact-timing recheck

On 2026-11-13, raylib-rs was built from `1d83e7401d730e0d5a469019d10e06d269e9a87c` with locked offline dependencies in a fresh Windows MSVC debug target. Three adapter tests passed. Build and test links emitted `LNK4098` for conflicting `MSVCRTD` default libraries; this warning was not suppressed or investigated in this run. Executable SHA-256: `19b4b2360c3601037995b5d180cc62c6c634c96c047f019962511b0e1692b968`.

The scripted replay exited 0 at tick 1440 with empty stderr and six app-owned PNGs. Its 13 [telemetry records](evidence/windows-raylib-contact-timing-20261113.jsonl), filtered from raylib's mixed stdout, matched the contact-timing model reference line-for-line. The paused tick 480 capture was inspected and shows the court, score, paddles, ball and pause text. This is an app-owned capture, not a desktop screenshot or proof of OS input.

No new normal interactive test was performed. The earlier Windows supported start-input limitation remains open; physical keyboard, full match, focus/resize, packaging and release qualification remain unverified for this revision. Previous binaries and captures remain historical.
