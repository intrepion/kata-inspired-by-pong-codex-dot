# Slint Linux portable package pilot

2026-11-12, Omarchy ARM64. Fresh offline locked optimized build from game-source checkpoint 730881f completed in 1m58s. Four shared GUI-model tests and 28 shared Node tests pass. No changes to game behavior or system settings.

## Verified

The 8,875,475-byte archive was extracted into a fresh directory. All five checksum entries passed, including an exact copy of the existing Slint license document. The relocated native event loop completed scripted replay with exit 0, 13 states through tick 1440, finished=true and six Slint own-window component snapshots.

Tick 600 was inspected: readable title, score, controls, paddles, ball, court and visible AboutSlint attribution. These are component snapshots, **not compositor screenshots** or OS-input evidence.

Exact six-file uninstall preserved the original release build, archive, snapshots and neighboring sentinel. [Archive/binary/license digests and runtime result](slint-linux-package-20261112.json).

## Dependencies and remaining gates

The root MIT license does not relicense Slint. The existing Slint Royalty-free Desktop, Mobile, and Web Applications License 2.0 text is included as SLINT-LICENSE.md; existing AboutSlint attribution remains visible. This is not a completed license review for every transitive dependency.

Direct ELF dependencies include fontconfig, libc, libm and libgcc. Dynamically loaded platform libraries and display services are not exhaustively described by that list and are not bundled. A clean-machine, different distro or architecture was not tested.

This qualifies local archive extraction/runtime/removal only. Real keyboard/release/focus behavior, compositor styling, public installers, signing, upgrades and complete notices remain separate gates.
