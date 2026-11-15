# GPUI Linux ARM64 local package pilot

## Scope and dependency limits

The Linux portable helper recognizes GPUI explicitly as **state-only**: this adapter rejects `--scripted-captures` rather than exposing desktop screen capture or fabricating pixels. Its archive includes the Pong MIT license and the exact pinned GPUI 0.2.2 Apache license separately. [License provenance](../../experiments/gui-gpui/GPUI-LICENSE-PROVENANCE.md).

[Notice feasibility scan](gpui-linux-notice-feasibility-20261112.json): 548 packages in the resolved Linux ARM64 Cargo graph, 932 discovered notice texts, and 30 packages with no discovered conventional notice file. This is a graph scan, not proof of linked attribution, a license violation, or license clearance. Full transitive notice review remains open. The incomplete bundle is not packaged as cleared.

Existing system graphics/font/display libraries remain external prerequisites. No drivers, settings, services, runtime library installation or public release is part of this local pilot.

## Verified local lifecycle, 2026-11-12

Fresh locked offline optimized build completed in 3m32s with eight shared GUI-model tests passing. Existing `proc-macro-error2` future-incompatibility warning remains. Sixteen applicable packaging tests pass (two Windows-only cases skipped), including eighteen adapter fixture paths and GPUI's state-only license-bearing guide.

The new archive was extracted separately, all five checksum entries verified, and the extracted native GPUI process completed thirteen replay states through tick 1440 with exit 0. Unsupported capture returned 2 and created no output directory. Only six exact archive-owned files were removed after execution; the now-empty extraction folder was removed and an adjacent sentinel preserved. Original binary/archive/logs remain local.

[Redacted manifest](gpui-linux-package-20261112.json). Direct ELF dependencies include FreeType, XCB, xkbcommon/xkbcommon-X11, libgcc, libm and libc; graphics libraries may also be loaded dynamically. This is not an exhaustive runtime inventory or a static/self-contained build. No pixel/visual/input pass is claimed.
