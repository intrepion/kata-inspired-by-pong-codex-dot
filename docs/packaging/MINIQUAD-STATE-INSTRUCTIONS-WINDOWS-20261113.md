# Miniquad state instructions: Windows check

At checkpoint `3d0c009`, Miniquad adapter blob `6e5487d7a7a1d58d44ce4d7b5c8e59a4311f7a89` passed all 15 locked offline Windows MSVC debug tests, including state geometry and mapped key actions. A fresh locked offline debug build produced SHA-256 `dc5c3e5b6091d5276a4c16fbf364098a1eb63a645af9a19bb1b08141bd2ed55b`.

The app-owned scripted run exited 0 with empty stderr; all 13 JSON records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six framebuffer captures were produced. The inspected tick-480 capture (SHA-256 `d4f11bf7758690a7eed551b8d09946e3c7070f86a6baf300d65be91cb14de431`) shows pause bars and the complete `SPACE TO RESUME` prompt. Previous executables, logs and captures remain in sibling workspace directory `miniquad-prompt-windows-20261113`.

The standard replay does not capture Ready or game over. Mapped-action unit tests are not evidence of OS key delivery: the previously observed Windows automation `Unknown` key distinction remains unresolved. No physical input, complete interactive match, clean-machine operation, license clearance or release qualification is claimed.
