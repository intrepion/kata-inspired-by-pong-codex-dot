# Windows portable ZIP batch 3

| Adapter | Extracted replay seconds | States / PNGs | Hashes |
| --- | ---: | --- | --- |
| Nannou | 13.814 | 13 / 6 | [Manifest](nannou-windows-manifest-20261112.json) |
| bracket-lib | 12.710 | 13 / 6 | [Manifest](bracket-lib-windows-manifest-20261112.json) |
| Fyrox | 13.579 | 13 / 6 | [Manifest](fyrox-windows-manifest-20261112.json) |
| Bevy (DX12 only) | 16.408 | 13 / 6 | [Manifest](bevy-windows-manifest-20261112.json) |
| Blue Engine (native state only) | 13.460 | 13 / 0 | [Manifest](blue-engine-windows-manifest-20261112.json) |

All local archives created/extracted in absent-before-run directories, including a space in each extracted working directory. Archive includes executable, project LICENSE and explicit incomplete-pilot warning. PowerShell literal paths, Compress-Archive and Expand-Archive; extracted SHA-256 matches source. Ran from extracted folders, all exit0/finished tick1440. No system install, registry/PATH changes, security bypass, public binaries or deleted staging/build files.

Bevy uses process-only `WGPU_BACKEND=dx12`; default Vulkan remains rejected, so a default-launch Windows portable release is NOT qualified. Blue native replay has no screenshot mode and uses existing forced exit; no graceful native teardown or visual pass is inferred. Other four tick600 captures inspected: paddles, ball, score and divider visible, Bevy/ bracket controls readable. Nannou has no control text in this frame. Fyrox has complete readable small header at800x600. Bracket was rebuilt with divider contrast correction; nine release tests pass. Fyrox uses corrected HUD binary. Earlier timing reports keep their original binary provenance.

Fyrox pre-run read-only receipt confirmed no registry in its isolated extracted folder; afterwards `data/resources.registry` contained exact two-byte `{}`. Inspection returned review-candidate, not deletion authorization. The generated state is preserved. Older checkout-generated data is also untouched. This is not clean uninstall.

All fifteen currently timed native adapters now have Windows local portable archive/extracted-runtime evidence, but not universal release readiness. Full notices, dependency/clean-machine tests, signed/downloaded-file behavior, actual installers/upgrades/uninstalls and native keyboard/focus gates remain. No new benchmark or ranking claim.
