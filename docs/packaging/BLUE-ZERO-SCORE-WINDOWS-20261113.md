# Blue Engine zero-score geometry: Windows, 2026-11-13

Blue Engine draws score pips, but previously an empty group drew no score. The adapter now supplies four existing-style rectangles for an outlined zero when the score is zero, hiding them for a positive score while retaining the pips. It preallocates those rectangles rather than allocating objects each frame. Gameplay and input are unchanged.

On Windows MSVC stable Rust 1.99.1, based on checkpoint `01ee305f0f1e7da5410d077e2f4be204deca019e`, all 13 locked offline release tests passed, including a model-level geometry check for zero and positive scores. The adapter source Git blob is `d24364765c6ddd5a7699395a6211dcc9229e59b1`; the windowed release executable SHA-256 is `582cda7462ec785dde99c286718d2a72a92f9b9afa6635ddc2ba8b5426e19c1a`. Its app-owned scripted run without capture exited 0 with empty stderr and all 13 parsed telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`.

The default windowed build does not support app-owned capture. A separate headless-capture build began a broad dependency rebuild and was stopped to bound disk use; its partial target was preserved. No new visual verification is claimed. Logs and copies of prior windowed and headless binaries remain in sibling workspace directory `blue-zero-score-windows-20261113`.

Actual zero presentation, other scores and platforms, physical and accessibility input, complete dependency and system-library review, clean-machine installation and release qualification remain open. This is not license clearance.
