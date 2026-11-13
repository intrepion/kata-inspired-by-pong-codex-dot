# Softbuffer current Windows release pilot

At checkpoint `c566b149ab7142b46b411d22831e68f00af50f6b`, Windows stable MSVC Rust built the softbuffer adapter offline and locked in release mode; all ten optimized adapter tests passed. Adapter source blob was `c46cb48675475fb145752446e58a740e8d78b3b0`, shared renderer blob `dcb11dadcc350bb43655a760ced8c42d0128b3dd`, and executable SHA-256 `c1eeb67a45445394e23ba84ffd43da79552e2943c6071788ad90db28c7a6b9c3`.

A fresh Windows-filtered locked Cargo metadata inventory reported 35 resolved packages, 68 discovered notice files and no missing-file flags; bundle SHA-256 `da5ff4eeb02f2f6bbc14be3419b7c16a4aa5283f7a34a7b4ddcda44de97ab77c`. The declared license expressions did not match LGPL, GPL-3 or MPL in this graph. Neither this scan nor the absence of flags proves final binary attribution, complete bundled/system-library review, or legal clearance.

A new local unsigned 396,074-byte ZIP (SHA-256 `a3cad016703adbf35da979803c9099e9d7a16631312caac65c50e968de669f88`) contains the executable, project license, notice preparation text and index, incomplete-pilot note, and SHA-256 manifest (manifest SHA-256 `5e5fef6cedcee41baf7873ccafbfb73d12850b2246c09df8af799c144b0ee586`). All five payload hashes matched after extraction into a separate path containing a space.

The relocated executable exited 0 with empty stderr; all 13 JSON records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were generated. Tick-480 SHA-256 `fdede478ed6af22eb32a50139a82abf6c097306a6d12089f503ae818fd8baa31` is byte-identical to the previously inspected [score-label capture](../evidence/softbuffer-ash-score-labels-windows-20261113.png). Archive, staging, extraction, prior executable, notice inventory, logs and captures remain in sibling workspace directory `softbuffer-current-release-windows-20261113`.

This is same-host extraction and scripted rendering, not OS key delivery, physical input, complete gameplay, a clean-machine install, signed/downloaded-file verification, dependency-license clearance or release qualification. No public release was made.
