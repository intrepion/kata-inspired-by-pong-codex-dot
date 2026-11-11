# Local compatibility backport

Upstream: PistonDevelopers/freetype-rs 0.26.0, MIT (LICENSE retained).
https://crates.io/crates/freetype-rs/0.26.0

This copy changes only src/bitmap.rs behavior plus three regression tests: a zero-length bitmap returns an empty slice rather than calling slice::from_raw_parts on FreeType's null buffer. Matches the empty-buffer handling present in official freetype-rs 0.38.0 (bitmap.rs lines 63–70, checked 2026-11-11). No unsafe precondition checks are disabled.

Why backport: bracket-lib 0.8.7's crossfont dependency requires freetype-rs 0.26. No 0.26.1 exists on crates.io. Jumping directly to 0.38 is not semver-compatible and would require upgrading/forking additional windowing dependencies. Keep this narrow patch until the dependency chain can upgrade. macOS and Windows crossfont do not use this FreeType dependency.

Regression tests cover null empty glyph, null zero-pitch glyph, and a populated two-row glyph. Original test abort reproduced before patch. Explicit native Wayland launch passed eight-second smoke test after patch. Visual gameplay remains unverified.
