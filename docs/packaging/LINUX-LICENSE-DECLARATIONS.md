# Linux packaging license-declaration audit

2026-11-12, metadata-only follow-up to the local portable pilots.

Ten first-party adapter manifests lacked a license declaration even though the repository root LICENSE already licenses this code under MIT. Added the consistent MIT package field for Blue Engine, softbuffer, Miniquad, pixels, raw wgpu, Piston, ggez, Nannou, bracket-lib and Fyrox. This does not relicense any third-party package.

Offline, locked Cargo metadata for the Linux ARM64 target resolves successfully for those ten plus raylib-rs. The [summary](linux-license-declaration-audit-20261112.json) records package counts and missing declarations: none remain in these eleven resolved graphs. The existing inventory helper's three regression tests pass. No dependency versions, lockfiles, game behavior or archive binaries changed.

This is only declaration hygiene, not a completed license-notice bundle or legal clearance. Graphs can include build/optional dependencies rather than only linked code. Next release gates are collecting and checking actual license text, attribution and bundled native-source notices, then clean-machine/system-runtime compatibility. Existing local archive checks do not satisfy those gates.
