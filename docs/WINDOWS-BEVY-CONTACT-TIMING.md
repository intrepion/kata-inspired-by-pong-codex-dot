# Windows Bevy contact-timing recheck

On 2026-11-13, Bevy was built from `87818d005cac3bf3b6201cc6925971f10511d940` with locked offline dependencies in Windows MSVC debug mode, in a fresh target directory. Both adapter tests passed. Executable SHA-256: `c1ba19db0e32e3cfb4582951bb3d98d0dc78b4f0c2ce90a3cca905b45e53bfd3`.

With `WGPU_BACKEND=dx12`, the scripted replay exited 0 at tick 1440 and produced 13 [telemetry records](evidence/windows-bevy-contact-timing-20261113.jsonl), matching the contact-timing model reference line-for-line, and six app-owned PNGs. The inspected tick 480 frame shows the pause message and score 0:1; tick 1320 shows score 0:0 without that message. These captures are app-owned images, not desktop screenshots or evidence of OS input. The renderer reported Intel UHD Graphics 630 with DX12 and driver 31.0.101.2145.

The historical default Vulkan failure described in [the diagnosis](WINDOWS-BEVY-DIAGNOSIS.md) was not retested. Normal native interaction, physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Previous binaries and captures remain historical.
