# Windows dependency-notice feasibility

Offline locked Cargo metadata filtered to x86_64-pc-windows-gnu, then shared notice-file collection. Resolved graph is not precise linked-code attribution; build/optional crates may be included. No packages were downloaded, replaced, or given invented license text.

| Adapter | Packages | Notice files | Packages missing discovered files |
| --- | ---: | ---: | ---: |
| miniquad | 18 | 33 | 1 |
| softbuffer | 35 | 68 | 0 |
| sdl3 | 34 | 113 | 0 |
| ash | 36 | 69 | 0 |
| raylib-rs | 52 | 104 | 1 |
| pixels | 117 | 215 | 8 |
| wgpu | 113 | 210 | 7 |
| piston | 213 | 368 | 10 |
| ggez | 171 | 302 | 14 |
| nannou | 424 | 720 | 31 |
| bracket-lib | 77 | 130 | 8 |
| fyrox | 393 | 628 | 34 |
| blue-engine | 202 | 352 | 14 |

| macroquad | 48 | 92 | 2 |
| bevy | 349 | 633 | 17 |

[Exact missing-package lists](windows-notice-feasibility-20261112.json). Only softbuffer, SDL3 and ash have discovered files for every package in these selected graphs. Even these are not legal clearance: nested/unlabelled bundled code, selected licensing alternatives, native DLLs/system libraries and actual linked content still need review. Miniquad specifically lacks a discovered file for winapi-x86_64-pc-windows-gnu0.4.0; do not borrow its Linux clearance.

Root-workspace Macroquad and Bevy were subsequently scanned using only their transitive resolved dependency closures, including build dependencies, rather than every unrelated workspace member. Both have discovered-file gaps. All fifteen native timing adapters are now represented. No notice-bearing Windows archive or public release is claimed by this report. Existing local ZIP pilots explicitly remain incomplete.

Follow-up: three new [notice-bearing local ZIPs](WINDOWS-NOTICE-PACKAGES.md) now include discovered texts and hash indexes for softbuffer, SDL3 and ash. This supersedes only the earlier no-bundle preparation status, not the stated attribution, legal, system-library or release limitations.
