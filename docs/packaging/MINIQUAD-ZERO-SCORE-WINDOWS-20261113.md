# Miniquad zero-score visibility: Windows, 2026-11-13

Miniquad uses tally pips for scores; previously zero drew no mark, making an empty group indistinguishable from a missing score. It now draws an outlined zero for an empty group and retains the existing pips for positive scores. Gameplay and input are unchanged.

On Windows MSVC stable Rust 1.99.1, based on checkpoint `fd08dd90c5511d46b400c7b58538008a425c497b`, all 16 locked offline release tests passed, including a score-geometry regression for zero and positive groups. The adapter source Git blob is `3541f3198e3d7e529d24711fdab502f00bacb890`; the release executable SHA-256 is `0d43c1a8f015824f33b4072f5e3e81f48a26c8dff7957718c8927627cef58d9e`.

Its app-owned scripted replay exited 0 with empty stderr and all 13 parsed telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The inspected tick 480 capture at 0–1 showed a left zero and a right pip; SHA-256 `91a2d6808a6c4d80eb6f70e67153a1068c9894dbd48d472541c3b0e1c1ee7f73`. The inspected tick 1320 capture at 0–0 showed two zeros; SHA-256 `06d528e5084b16de41ac7b2bcf283018dc97cab503c0fe9cf1391ad67ad112a1`. Prior executable and test copies, both replay attempts and captures remain in sibling workspace directory `miniquad-zero-score-windows-20261113`.

These are app-owned scripted captures, not physical input or a full interactive match. Other score combinations, sizes and platforms, native keyboard delivery, accessibility, complete dependency and system-library review, clean-machine installation and release qualification remain open. This is not license clearance.
