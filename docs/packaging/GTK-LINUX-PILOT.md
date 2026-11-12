# GTK Linux portable package pilot

2026-11-12, Omarchy ARM64. Fresh offline locked release build from game-source checkpoint 906fe03 completed. GTK's Cairo snapshot regression test and all four shared GUI-model tests pass; 28 shared Node tests pass.

The 335,725-byte archive was extracted into a fresh isolated directory. All four SHA-256 manifest entries passed; the relocated executable completed its native GTK event-loop scripted run with exit 0, 13 states through tick 1440, finished=true and six app-owned Cairo PNGs. Tick 600 was inspected: title/score, paddles, ball and court are visible. Exact five-file uninstall preserved the neighboring sentinel, original binary, archive and captured evidence.

[Archive hash, binary provenance and direct shared-library list](gtk-linux-package-20261112.json).

## Runtime dependencies and evidence limits

GTK runtime 4.24.1 was already present. The ELF directly requires GTK4, Pango, GdkPixbuf, Cairo, GLib/GObject/GIO, libc, libm and libgcc; their libraries are not bundled. A small archive therefore does not mean a self-contained application. No system libraries were installed or modified.

The exported PNG is an app-owned Cairo rerender from the model using the same drawing routine, **not a compositor screenshot**. It does not establish actual keyboard input, focus handling, window placement or desktop styling. The GUI-model tests cover release/focus cancellation and bounded catch-up but do not replace OS-input validation.

This is local extraction/runtime/removal, not a clean-machine, cross-distro, public installer, signing, upgrade or dependency-notice qualification. The shared packager explicitly labels GTK's runtime requirement and capture type.

## Corrected GUI input model requalification (2026-11-12)

Fresh offline locked optimized build from `fca3d4d` includes both held-alias and case-changing key-identity corrections. Eight shared model tests pass. A new archive was extracted into a fresh directory; every manifest hash passed. The extracted native scripted run exited 0 with 13 states through tick 1440, finished=true, and six app-owned captures. Tick 600 was inspected: readable score, court, paddles, ball and controls. Sixteen packaging safety tests pass; two Windows-only tests skip.

[Redacted hashes and results](linux-gui-identity-followup-20261112.json). The new archive, extraction and logs are retained; this follow-up does not repeat uninstall qualification or replace historical hashes. Scripted replay does not prove actual OS-keyboard alias/release behavior. Capture remains Cairo render output, not compositor capture. Dependency, clean-machine and public-installer gates remain open.
