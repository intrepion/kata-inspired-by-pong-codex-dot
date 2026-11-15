# SDL3 focused Escape: Windows cross-check

At checkpoint `3d0c009`, SDL3 adapter source blob `d7c995ac1f91f28dae8e74c02a69f9a58ddc0109` passed all 12 locked offline Windows MSVC debug tests, including the constructed-event regression for queued Escape after focus loss and Quit with either focus state. A fresh locked offline debug build produced executable SHA-256 `94525311ffcf626ae204b98a21aa4e246f37e0243d3d220c9ae3db1bca74b71d`. Both ordinary Cargo test and build retained the known `LNK4098` conflicting CRT default-library warning; it was not suppressed.

The scripted run exited 0 with empty stderr and all 13 JSON records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Previous executables and logs remain in sibling workspace directory `sdl3-focus-escape-windows-20261113`.

The regression constructs adapter events; the scripted replay does not exercise OS focus transitions or physical keyboard delivery. No complete interactive match, clean-machine operation, license clearance or release qualification is claimed.
