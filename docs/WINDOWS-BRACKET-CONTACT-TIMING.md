# Windows bracket-lib contact-timing recheck

On 2026-11-13, bracket-lib was built from `294b12aa054bdc880da93cef1d03d4df07e35cab` with locked offline dependencies in a fresh Windows MSVC debug target. All nine adapter/model tests passed. Executable SHA-256: `927a39aaa6430b519387ce2736ecc997816f7efb39dab68b004f8d6ce2eff723`.

The scripted replay exited 0 at tick 1440 with empty stderr and six app-owned PNGs. Its 13 [telemetry records](evidence/windows-bracket-contact-timing-20261113.jsonl), filtered from stdout, matched the contact-timing model reference line-for-line. The paused tick 480 capture was inspected and shows the court, score, paddles, ball and pause status. This app-owned capture is not a desktop screenshot or proof of OS input.

No normal interactive test was performed. Native and physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Previous binaries and captures remain historical.
