# ash Vulkan / Linux ARM64 local portable lifecycle

Verified 2026-11-12 on Omarchy ARM64. Tested executable source is checkpoint
`5ae985f4726def423650741feaf5e928f4894ebe`, the native timing/replay-qualified
build. Packaging is application/platform-specific, not part of pong-core.

| Check | Result |
| --- | --- |
| Bash syntax and three package guard tests | Pass; test fixture is /bin/true, not a runtime qualification |
| Portable archive creation | Pass, 1,647,344 bytes, five files |
| Archive and extracted file SHA-256 checks | Pass |
| Fresh task-owned extraction outside checkout | Pass, no system installation |
| Relocated native replay from /tmp | Exit 0, thirteen states through tick1440/finished=true |
| Relocated app-owned Vulkan captures | Six PNGs; tick600 inspected with paddles, ball, score and net |
| Exact-file uninstall | Pass; only five expected installed files removed, empty package directory removed |
| Neighboring sentinel and original executable preservation | Pass |
| Existing-output protection | Pass in guard tests; no overwriting |
| Driver/loader installation, real keyboard/focus, clean-machine/distro compatibility | Not run |
| Signed installer, upgrade, complete dependency notices | Not qualified |

Archive SHA-256: `0410deee3aa2e0a2863676140d2153b8fdb787c1631f92336eac797f17848e2e`.
The archive stays local and is not committed or publicly released. An initial
local archive had an inaccurate uninstall file-count sentence; the corrected
archive above was regenerated into a fresh output and is the qualified run.

Reproduce with `bash experiments/ash/packaging/linux-portable.sh RELEASE_BINARY
NEW_OUTPUT_DIRECTORY SOURCE_COMMIT`. This records supplied provenance and
checksums, but cannot prove an arbitrary input executable came from that commit.
Verify SHA256SUMS after extraction before running. The packager refuses an
existing output directory, bad commit format, directory or nonexecutable input.

Direct runtime links include glibc, libm and libgcc_s; Vulkan loader/driver and
window-system libraries are also runtime requirements. Static link inspection
is not a complete dependency inventory. No driver is bundled, installed or
reconfigured. Root MIT is included; transitive notices remain a release gate.

This local extraction/removal pilot creates no service, desktop entry, PATH
change or package-manager registration and does not qualify a public installer,
another Linux distribution or architecture. The portable instructions contain
an exact-file removal recipe rather than recursive deletion of arbitrary paths.
