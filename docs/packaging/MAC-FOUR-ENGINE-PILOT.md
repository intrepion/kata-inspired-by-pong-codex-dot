# Four more Mac local package pilots

2026-11-12, ARM64 macOS. Existing qualified release binaries reused without
compilation. Source base `138427d`. Exact binary/archive hashes and results:
[Nannou/bracket-lib](mac-nannou-bracket-package-20261112.json),
[Bevy/Fyrox](mac-bevy-fyrox-package-20261112.json).

All four minimal app bundles passed plist validation and archive extraction.
Extracted binary hashes matched their source manifests. Each relocated native
scripted capture run exited zero, emitted thirteen checkpoints through finished
tick 1440, and saved six PNGs. Elapsed lifecycle times were Nannou 22.564 seconds,
bracket-lib 13.830, Bevy 17.628 and Fyrox 16.473. These include startup and are
not controlled benchmark measurements. No heavy build or GUI automation occurred.

Tick 600 screenshots were inspected for all four. Court, paddles, ball and score
were present. Preserve known presentation caveats: bracket-lib's divider is very
low contrast; Fyrox's top text is small compared with its court. Nannou lacks an
instruction HUD in the sample. No full visual or keyboard acceptance is inferred.

Only each generated extracted app was removed; adjacent sentinel preserved.
Archives, original binaries and capture evidence remain. All 36 package guard
tests passed across twelve Mac scripts with real binaries, including destination
overwrite protection. All test processes exited.

No public installer, signed/notarized distribution, quarantine/Gatekeeper,
complete third-party notices, clean-machine dependency check, upgrade, native
real-input package test or Intel qualification is claimed. No security settings
were changed. Approximately 17 GiB remained free during the batch.
