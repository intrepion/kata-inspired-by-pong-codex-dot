# bracket-lib Windows held-input release portable pilot

At checkpoint `849c5eb680c1d43f7e58e8f69c4af7fc523d4e5c`, including held-input pause fix `5fb4cc675ca039466422fe10288efe98375a7835`, the bracket-lib adapter was built offline and locked for Windows MSVC release with Rust 1.99.1 in a fresh target directory. All 11 optimized tests passed. Source blob `5cd7e8315692fae800b7de0c641fecefa90c13ad`; executable SHA-256 `7814bcfaa44113bd822734b94e9c97ca027daea6beafb827cbc9d75f3656f997`.

A Windows-filtered locked Cargo metadata scan with the corrected notice collector found 76 resolved packages and 130 conventional notice files. Seven packages were flagged without a conventional notice file: `bracket-embedding@0.8.7`, `bracket-noise@0.8.7`, `bracket-rex@0.8.7`, `gl_generator@0.14.0`, `khronos_api@3.1.0`, `ultraviolet@0.9.2`, and `windows_x86_64_msvc@0.36.1`. The incomplete local bundle SHA-256 is `332b66b00a1e7165aa51bb6dc369da5142cdda5a041fec59c6ccae36ac552e87`. The resolved graph may include build or optional packages; flags do not establish binary linkage or license obligations. The incomplete bundle was excluded from the package.

The unsigned 715,001-byte local ZIP SHA-256 is `a99d53d770f2a65443227c1a38de56e76f491bc13d736dd275c63c64b2414438`. It contains the executable, project license, incomplete-pilot note, and SHA-256 manifest. Extraction to a separate path containing a space preserved all three payload hashes; manifest SHA-256 `e05f465262a3273b6b27f035a90fd4213e351ecb810a32f5c7f9a02df055b573`.

The relocated executable exited 0 with empty stderr; all 13 scripted telemetry lines matched `../evidence/rust-core-contact-timing-model-20261112.jsonl` through finished tick 1440. Six app-owned PNGs were exported. The inspected tick-480 image shows the paused label, court, paddles and ball. The extracted package retained only its four staged files. Archive, extraction, raw logs and captures remain in task workspace `bracket-held-release-package-20261113`; metadata and incomplete notice bundle remain in `bracket-windows-held-notices-20261113`. Prior packages and binaries are preserved.

This is same-host portable extraction and scripted rendering, not a desktop screenshot or actual OS-key delivery. Physical input, accessibility, full match, focus/resize, clean-machine installation, system dependencies, signing, notice completeness and release distribution remain open.

## Notice label correction

A prior correction here incorrectly said the retained incomplete bundle had a Linux heading. Reinspection confirmed its heading is Windows and its SHA-256 is the originally recorded 332b66b00a1e7165aa51bb6dc369da5142cdda5a041fec59c6ccae36ac552e87. It remains excluded from the pilot package because seven notice flags were unresolved; no historical artifact was replaced.
