# Windows portable ZIP batch 2

| Adapter | Extracted scripted seconds | States / PNGs | Exact hashes |
| --- | ---: | --- | --- |
| SDL3 | 12.509 | 13 / 6 | [Manifest](sdl3-windows-manifest-20261112.json) |
| ash | 13.266 | 13 / 6 | [Manifest](ash-windows-manifest-20261112.json) |
| raylib-rs | 12.676 | 13 / 6 | [Manifest](raylib-rs-windows-manifest-20261112.json) |
| Piston | 13.353 | 13 / 6 | [Manifest](piston-windows-manifest-20261112.json) |
| ggez | 13.534 | 13 / 6 | [Manifest](ggez-windows-manifest-20261112.json) |

Reused the release executables identified by these hashes. Each archive contains executable, project LICENSE and explicit local-pilot warning. New-only stage and extraction paths containing spaces; PowerShell Compress-Archive/Expand-Archive with literal paths. Extracted hashes match sources exactly. Ran from extracted working directories; every run exited0 and finished tick1440. No registry/PATH/security changes, no archive distribution or deletion of active build/staging files.

Tick600 images individually inspected: court, both paddles, ball, centerline and score indicators present. ggez text HUD is readable. SDL3/ash use numeric score; Piston retains tally-style score and lacks a text control legend. raylib footer remains low contrast and overlaps the paddle region, a known visual limitation, not a clean visual pass. This does not qualify native OS-input or all display sizes.

These are local portable extraction/runtime pilots, NOT clean-machine, installer, upgrade/uninstall, signed/downloaded-file or complete third-party-notice qualifications. Installed GPU/driver/runtime dependencies on this developer computer can mask missing dependencies. Existing timing evidence remains separate.
