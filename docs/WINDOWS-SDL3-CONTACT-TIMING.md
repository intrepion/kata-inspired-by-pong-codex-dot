# Windows SDL3 contact-timing recheck

On 2026-11-13, SDL3 was freshly built with locked offline dependencies in a separate Windows MSVC debug target from `617160ceb4573ff06f7339c7dbc98787d54f55df`. Both SDL3 adapter tests passed. The build and test links emitted `LNK4098` about conflicting `MSVCRTD` default libraries; this warning was not suppressed. Executable SHA-256: `299eb8a88e5ffcb9495b568b80e87723dac4a15a388f91d4e0290bcec122e723`.

The scripted replay exited 0 at tick 1440 with empty stderr, 13 [telemetry records](evidence/windows-sdl3-contact-timing-20261113.jsonl), and six app-owned PNGs. Discrete fields matched the contact-timing model reference and maximum absolute numeric difference was zero (allowance 0.001). The paused tick 480 capture was inspected and shows the court, score, paddles, ball and pause marker. This renderer capture is not a desktop screenshot or proof of OS input.

No normal interactive test was performed in this follow-up. Physical keyboard, native input, full match, focus/resize, package and clean-machine qualification remain open for this revision. The linker warning warrants separate investigation before release. Previous binaries and captures remain historical evidence.
