# Windows polling non-playing movement recheck

At checkpoint `a5667faa7773ba32b22b066d0e6007ca6e9da0ed`, which includes `04b0cbd`, Windows MSVC stable Rust 1.99.1 ran Macroquad's locked offline debug test suite: all three movement regression tests passed. The tested `clients/macroquad/src/main.rs` Git blob is `80b9fb91ad84c313bf45b0d2f803c94987d8bec9`.

A fresh locked offline debug build produced `pong-macroquad.exe` SHA-256 `f23d0bc9a40e07cedf64c6a8b5427d866835cf0bf640939a30ce490e74a44c16`. Its scripted run exited 0, stderr was empty, and all 13 parsed telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. No capture or native key interaction was tested. Executable and partial build dependencies remain under `target-polling-nonplaying-windows-20261113`; output remains in the sibling workspace directory `polling-nonplaying-windows-20261113`.

Nannou's fresh test compilation was stopped after several minutes while still compiling graphics dependencies; no Nannou test result is claimed. Raylib and SDL3 were not run in this batch. Model tests and scripted replay do not establish physical key delivery, accessibility, clean-machine operation, license clearance or release readiness.
