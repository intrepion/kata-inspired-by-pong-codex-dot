# Verification status (2026-11-11)

## Passed on Mac arm64

- Official Godot4.5.1 binary reports `4.5.1.stable.official.f62fdbde1`.
- Rust full debug build, single job, pinned godot0.4.5/API4.5 (Cargo.lock checked in with source when authorized).
- GDExtension loaded with strict runtime safeguards.
- Headless `--script smoke.gd`: extension instantiate, Ready/start, move, pause freezes movement, resume, reset scores and paddle position all pass.
- Actual main scene headless run for120 frames exits cleanly, no parse/runtime errors.
- Native graphical launch: court, paddles, ball, score and status render correctly. Computer Use exercised Return start, W movement, P pause, Return resume and R restart; visible paused state and moved paddle confirmed. Core reset assertions separately verified headlessly.

## Known issue

`--headless --editor --quit` scans/imports successfully, but on shutdown Godot4.5.1 crashes in `EditorHelp::_gen_extensions_docs` / `DocTools::generate`. The same project's game runtime and integration script exit cleanly. Do not mark editor workflow as passing. Investigate with a later pinned compatible Godot patch before distributing an editor-oriented package.

Initial godot0.4.0 failed compilation on installed Rust1.99 (upstream trait-object lifetime cast). Updated to pinned godot0.4.5, which builds successfully. No dependency source modified.

## Not yet qualified

Windows and Omarchy builds/runtime; iOS/Android exports/touch UI; resize, focus-loss input interaction and complete match via real UI. No export templates installed. No distributable export created. No commit, push, release or publication performed.

Mac engine installed separately at ~/Applications/PongComparisons/Godot-4.5.1/Godot.app. Native library in ignored project/bin. Godot generated cache ignored. Run instructions in README.
