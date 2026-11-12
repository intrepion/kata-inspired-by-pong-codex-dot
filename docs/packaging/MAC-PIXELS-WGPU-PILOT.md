# Mac pixels and raw-wgpu local package pilots

2026-11-12, ARM64 macOS; packaging source based on `0d88f85`. Existing
previously qualified release binaries were reused; no dependency rebuild.
[Sanitized manifests/results](mac-pixels-wgpu-package-20261112.json) record exact
binary/archive hashes, exit status and bounded replay results.

Both minimal app bundles passed plist validation, were archived with the repository
MIT license and pilot notice, extracted outside the checkout, and hash-verified.
Relocated native scripted runs each produced thirteen telemetry records through
finished tick 1440 and six PNGs: pixels 13.931 seconds; raw wgpu 12.704 seconds.
Both exited zero. Tick 600 screenshots were inspected: court, paddles and ball
visible. Pixels has no numeric/text HUD in this sample; raw wgpu uses a tally
mark instead of a text score. No presentation parity or full visual acceptance
is claimed.

The exact generated extracted app was removed in each case, while an adjacent
sentinel survived. Original binaries, archives and capture evidence remain.
All eighteen guard tests passed across six Mac package scripts, including refusal
to overwrite an existing destination with real ARM64 executable inputs.

These are local archive/relocation/replay/removal checks only. Native real-input
checks of these packages, Finder launch, downloaded quarantine/Gatekeeper,
Developer ID signing, notarization, complete bundled third-party notices,
clean-machine dependencies, upgrades and Intel builds remain unverified.
No security settings were changed and no public installer was released.
