# Raw wgpu zero-score visibility: Windows, 2026-11-13

The raw wgpu court uses tally bars for score. Previously a score of zero drew no mark, making it indistinguishable from a missing score. The shader now draws an outlined zero in an empty score group and keeps the existing bars for positive scores. No gameplay or input logic changed.

On Windows MSVC stable Rust 1.99.1, based on checkpoint `de573eec94150183d608130701ccd69f868fc690`, all 13 locked offline debug tests and a debug build passed. The changed shader Git blob is `706fc55207ba9042aad08c8e5765472f8a7c0b2a`; the executable SHA-256 is `b1e0dc07114717fba5b0af36a511d1f1dc10905bb5fc2df8d121b435f976e6b9`. Cargo fell back to copying incremental files when hard-linking failed.

The app-owned scripted run exited 0 with empty stderr and all 13 parsed telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The inspected tick 480 image at 0–1 showed an outlined zero on the left and the existing single bar on the right; SHA-256 `e750261bd214904bedc2a8dd8008c397f7666cc6b0cb86644e336aaccab16abf`. The inspected tick 1320 image at 0–0 showed two outlined zeros; SHA-256 `fde9a25e301eb3827615555ab362d68e592d9e74c2790feb426f88dadb4db1fb`. Prior and new binaries, logs and captures remain in sibling workspace directories `wgpu-keypad-windows-20261113` and `wgpu-zero-score-windows-20261113`.

These are app-owned captures from a scripted run, not physical input or a complete interactive match. Other score combinations, window sizes and platforms, accessibility, clean-machine installation, full dependency and system-library review, and release qualification remain open. This is not license clearance.
