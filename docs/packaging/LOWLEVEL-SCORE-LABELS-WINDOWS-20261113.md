# Low-level score labels: Windows SDL3

The shared low-level renderer previously displayed two unlabeled score digits while using the same paddle color for both sides. It now draws `YOU` above the left score and `CPU` above the right score, independently of the window title. SDL3, softbuffer and ash use this shared rectangle list; no physics or input behavior changed.

A raster regression for the two label regions failed before the change and passed afterward. On Windows stable MSVC Rust, locked offline shared tests passed all ten cases (six unit, four integration), and all 12 SDL3 adapter tests passed. Cargo reported incremental hard-link fallback; ordinary SDL3 test and build retained the known `LNK4098` CRT warning.

The rebuilt SDL3 debug executable SHA-256 was `910de1090ee200425014adfd7d9e29b6810ce2821c5b674cecefe718385747d7`; shared source Git blob `dcb11dadcc350bb43655a760ced8c42d0128b3dd`. Its app-owned scripted run exited 0 with empty stderr; all 13 telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six captures were produced. The inspected [tick-480 capture](../evidence/sdl3-score-labels-windows-20261113.png), SHA-256 `d37d6da9fd6ce05ed7c02b21027da11d685e5393eae3f0705aa72cf51beef527`, shows `YOU 0` and `CPU 1` with the pause instruction. Prior executable, logs and captures remain in sibling workspace directory `lowlevel-score-labels-windows-20261113`.

This is shared raster and SDL3 app-owned scripted evidence at one size. Softbuffer and ash were not rebuilt for this change. Other window sizes, physical input, a complete interactive match, clean-machine operation, license clearance and release qualification remain open.
