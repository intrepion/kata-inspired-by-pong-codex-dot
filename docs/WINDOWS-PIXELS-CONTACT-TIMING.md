# Windows Pixels contact-timing recheck

On 2026-11-12, Pixels was built in a fresh Windows MSVC debug target from `4f4da1dc2fa82025786383dc1ea0ca38ba23ff7a` (the adapter, core and scenario match `c77bf3a`). The executable SHA-256 was `a72dcf4784f5f4d660a65bd9168617f90c4a9eb6bdc9346b1c6b19e02c77a36e`.

The scripted replay exited 0 at tick 1440 with empty stderr. Its 13 [telemetry records](evidence/windows-pixels-contact-timing-20261112.jsonl) matched the revised model reference in discrete fields and had maximum absolute numeric difference zero (allowance 0.001). Six app-owned PNGs were generated and inspected: each shows the court, paddles and ball. These are source-frame captures, not desktop screenshots or evidence of OS input.

A separate normal-launch request through supported Windows automation timed out awaiting approval; no interactive result is claimed. Keyboard movement, physical input, full match, focus/resize, packaging and clean-machine qualification remain open for this revision. Previous captures and builds remain historical evidence.
