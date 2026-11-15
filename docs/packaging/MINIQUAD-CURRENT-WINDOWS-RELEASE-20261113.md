# Miniquad current Windows release pilot

At checkpoint `b4d248d`, Windows stable MSVC Rust built Miniquad offline and locked in release mode; all 15 optimized adapter tests passed. Adapter source blob was `6e5487d7a7a1d58d44ce4d7b5c8e59a4311f7a89` and executable SHA-256 `32241758ffd4cd935d9b41dc541345752317e6c2f743aec80c197952c0728058`.

A fresh Windows-filtered locked Cargo metadata inventory reported 17 resolved packages, 33 discovered notice files and no missing-file flags; bundle SHA-256 `6128c162f605030e7beaf1269611ba3cf7a89e4c7b207617ec597cb4c4ef2daa`. No declared license expression in that graph matched LGPL, GPL-3 or MPL, and none was absent. This does not prove final binary attribution, complete bundled/system-library review or legal clearance.

A new local unsigned 281,765-byte ZIP (SHA-256 `9bd9d7d2480b667ee7ab639d262f89a621b988652c136e6255b784fc8c021275`) contains the executable, project license, notice preparation text and index, incomplete-pilot note, and SHA-256 manifest (manifest SHA-256 `a10c02502e6f799f567e4b1c8e8cf1a1d69699b8ebac2856518d64a7239daf66`). All five payload hashes matched after extraction into a separate path containing a space.

The relocated executable exited 0 with empty stderr and all 13 JSON records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were generated. Tick-480 SHA-256 `d4f11bf7758690a7eed551b8d09946e3c7070f86a6baf300d65be91cb14de431` is byte-identical to the previously inspected Windows pause-prompt capture documented in `MINIQUAD-STATE-INSTRUCTIONS-WINDOWS-20261113.md`. Archive, staging, extraction, prior executable, notice inventory, logs and captures remain in sibling workspace directory `miniquad-current-release-windows-20261113`.

This is same-host extraction and scripted rendering, not OS key delivery, physical input, complete gameplay, a clean-machine install, signed/downloaded-file verification, dependency-license clearance or release qualification. The prior supported-automation `Unknown` key observation remains unresolved. No public release was made.
