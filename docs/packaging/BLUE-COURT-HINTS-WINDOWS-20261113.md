# Blue Engine court action hints: Windows, 2026-11-13

Blue Engine now displays ENTER TO START, SPACE TO RESUME and ENTER TO RESTART in the relevant non-playing states, with a backing that keeps the hint readable against the court. The hint is hidden during play. Its glyph geometry is prepared once and reuses a fixed pool of rectangles; input and game physics are unchanged.

On Windows MSVC stable Rust 1.99.1, based on checkpoint `ba3f6cf4707df288a8a30174e216a05a1a524863`, all 15 locked offline windowed-feature release tests passed, including state-to-hint and glyph coverage. The adapter source Git blob is `5cc550a49b29d8c3c21cefc814480eb73660a773`. A locked offline `--release --no-default-features --features capture-headless` build succeeded; executable SHA-256 is `e406d753181431e6c40641881bbf892456df80abd6553779e81e74f24623fe4f`.

The app-owned scripted run exited 0 with empty stderr and all 13 parsed telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The inspected tick 480 capture shows SPACE TO RESUME under PAUSED, SHA-256 `283ffb35c2a773de1e377fc44d0e1dd94928318b5d34157ae450f8ab53b6134c`. The inspected playing tick 1320 image shows no hint, SHA-256 `a52cb8933a524591bea165e0264ee8bba42f1f67eb765dcec1ee98aa191ae43a`. Prior headless and windowed binaries, logs and captures remain in sibling workspace directory `blue-court-hints-windows-20261113`.

The replay does not reach Ready or game over, so those prompts have model/glyph tests but no Windows app-owned capture. Native-window presentation, physical and accessibility input, other sizes and platforms, performance impact, complete dependency and system-library review, clean-machine installation and release qualification remain open. This is not license clearance.
