# GPUI court aspect-ratio correction

2026-11-12, source base `a4721ce27ba73f5e9f070591aa1166b29bf4e9c8`.

The old GPUI court used independent relative horizontal and vertical sizes to fill its flex area. A 1000×300 court allocation therefore scaled world X by 1.1111 and Y by 0.6: the 900×500 playing field and circular ball were geometrically distorted. This is a source/geometry finding, not a claim of a newly observed desktop screenshot.

The renderer now paints the same game geometry into a centered, uniformly scaled GPUI canvas. Letterboxing preserves the 900:500 world ratio and the ball's equal-axis size; colors, physics, input and score layout are unchanged. Zero-width/height allocations produce no court painting.

## Verification on Omarchy ARM64

- A regression using the previous independent scaling failed (`1.1111112 != 0.6`). The corrected transform passes landscape, portrait, exact-fit and zero-size allocations, including centered offsets.
- One GPUI geometry test and all eight shared GUI-model tests pass.
- Strict all-target Clippy passes. Existing dependency future-compatibility warning for proc-macro-error2 2.0.1 remains; it was not suppressed.
- Fresh offline locked optimized build succeeds; its bounded native scripted run exits 0 with 13 states through tick 1440, finished=true.
- [Redacted binary/result record](evidence/gpui-court-aspect-linux-20261112.json).

GPUI app-owned pixel export is still unsupported. Native visual resizing/letterboxing and real keyboard input are **not yet verified here**; Mac follow-up requested. Existing package and timing hashes remain historical, not requalified by this renderer change. No desktop-control workaround was used.

## Mac qualification status

The existing Mac debug target was selected for an offline, single-job test/build attempt at source `5120220`. Cargo began recompiling a broad dependency graph rather than reusing a fully warm cache. The attempt was deliberately interrupted (exit 130) before tests completed to conserve disk and avoid a large background rebuild. This is **not** a compiler failure and **not** a Mac test pass. No native inspection of the corrected renderer is claimed; the older bundled executable cannot qualify the new aspect-fit change. Existing build files were preserved.

## Windows qualification

At source `5120220` (checkout `ccf6daf`), the optimized geometry test and all eight shared GUI-model tests pass. Strict all-target release Clippy and a fresh locked offline optimized build pass; the existing proc-macro-error2 future-compatibility warning remains. The freshly built native application completes its normal-window scripted replay with exit 0 and thirteen states through tick 1440, finished=true. [Binary/result record](evidence/gpui-court-aspect-windows-20261112.json).

A separate interactive launch was found by the supported desktop tool, but its app-access approval timed out. It was not retried; only that owned test process was stopped. No screenshot, native resize, physical input or clean-shutdown pass is claimed for the interactive attempt. Existing package and timing binaries remain historical.
