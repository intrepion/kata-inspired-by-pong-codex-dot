# Mac Piston and ggez local package pilots

2026-11-12, ARM64 macOS; packaging source based on `ed2ef73`. Existing qualified
release binaries reused with no compilation. [Sanitized manifests/results](mac-piston-ggez-package-20261112.json)
record exact hashes. Piston is the fixed wall-clock binary
`9ea7bd078b870d7bf66fd19242f74561c9fb0b1aaddcf7ecf3bd4e15ddbb8f2f`,
not the earlier slow-simulation build.

Both app plists validate. Archives contain minimal bundles, repository MIT license
and pilot notice. Fresh extraction outside the checkout produced executables
matching the manifests. Relocated native scripted capture runs exited zero:
Piston 14.544 seconds and ggez 13.894 seconds. Each emitted thirteen state records
through finished tick 1440 and six PNGs. These wall times are lifecycle observations,
not benchmark results or controlled comparisons.

Tick 600 images were inspected. Piston shows paddles, ball, divider and tally score;
ggez also displays numeric scores and instructions. This does not claim full
presentation parity, real-input or focus/resize acceptance. Extracted generated
apps were removed precisely, leaving adjacent sentinels and original archives,
source binaries and captures intact. No runtime remains active.

All 24 packaging guard tests passed across eight Mac scripts with real binaries,
including destination-overwrite protection. No signing/notarization, downloaded
quarantine/Gatekeeper, complete third-party notices, clean-machine dependency
validation, upgrades, Finder/manual-input package tests or Intel builds qualified.
No public installer released and no security settings changed.
