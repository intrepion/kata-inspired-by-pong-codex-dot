# Blue Engine score-owner labels: Windows, 2026-11-13

The Blue Engine court now labels its score groups YOU and CPU using existing glyph geometry. Label rectangles are created once during initialization; gameplay and input are unchanged.

On Windows MSVC stable Rust 1.99.1, based on checkpoint `6561509add5c131fed779b263f386ae163dfbd38`, all 14 locked offline windowed-feature release tests passed, including a label geometry check. The adapter source Git blob is `81e9e18e565e0b4aaef5b3db152322b6b7cb23a1`. A locked offline `--release --no-default-features --features capture-headless` build succeeded; its executable SHA-256 is `73019d7e200cadb1de840146b049a10b9a946b192f99ed979e10bb9873510cb5`.

Its app-owned scripted run exited 0 with empty stderr and all 13 parsed telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The inspected tick 480 image shows YOU/CPU labels and the zero/pip score at 0–1, SHA-256 `0d71e40e2174666111b697d31bbf734d4739c5450cf736c2538f5b471b4fedb7`. The inspected tick 1320 image shows both labels and zeros at 0–0, SHA-256 `a52cb8933a524591bea165e0264ee8bba42f1f67eb765dcec1ee98aa191ae43a`. Prior headless and windowed binaries, logs and captures remain in sibling workspace directory `blue-score-labels-windows-20261113`.

These are headless app-owned captures, not native-window or physical-input checks. Other scores, sizes and platforms, accessibility, complete dependency and system-library review, clean-machine installation and release qualification remain open. This is not license clearance.
