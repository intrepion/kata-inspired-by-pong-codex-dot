# Vulkan ash current Windows release pilot

At checkpoint `a27b35b31af629fd24e86627ee3b9d8cf3410fc7`, Windows stable MSVC Rust built the ash adapter offline and locked in release mode; all 12 optimized adapter tests passed. Adapter source blob was `4fa909f61c31e0036e3da0b267c287c4257a8bc4`, shared renderer blob `dcb11dadcc350bb43655a760ced8c42d0128b3dd`, and executable SHA-256 `5382dbfe5faa6413ec301f2b8f0697b7a09e8d1aab2d289bd0b2cd562cd7b054`.

A fresh Windows-filtered locked Cargo metadata inventory reported 36 resolved packages, 69 discovered notice files and no missing-file flags; bundle SHA-256 `53dd6b60bc9eaeb97dee46a97fd5ced3aa6c0bb27ca2a22d10ae6847218d5d71`. No declared license expression in that graph matched LGPL, GPL-3 or MPL, and none was absent. This does not prove final binary attribution, complete bundled or system-library review, Vulkan loader/driver licensing, or legal clearance.

A new local unsigned 408,198-byte ZIP (SHA-256 `3b6c8185dfabd48e7220bec835bfd98194eb72b2f87a4b22e573e893a7e6ea18`) contains the executable, project license, notice preparation text and index, incomplete-pilot note, and SHA-256 manifest (manifest SHA-256 `993bb62f3f915401e3f9b4f71680a3860adc78d5c52cc200d93412d31e5ef839`). All five payload hashes matched after extraction into a separate path containing a space.

The relocated executable exited 0; stderr reported Vulkan device Intel UHD Graphics 630, and all 13 JSON records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned Vulkan captures were generated. Tick-480 SHA-256 `fdede478ed6af22eb32a50139a82abf6c097306a6d12089f503ae818fd8baa31` is byte-identical to the previously inspected [score-label capture](../evidence/softbuffer-ash-score-labels-windows-20261113.png). Archive, staging, extraction, prior executable, inventory, logs and captures remain in sibling workspace directory `ash-current-release-windows-20261113`.

This is same-host extraction and scripted rendering, not OS key delivery, physical input, complete gameplay, a clean-machine install, signed/downloaded-file verification, dependency-license clearance or release qualification. No public release was made.
