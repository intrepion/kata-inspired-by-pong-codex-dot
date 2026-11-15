# Windows Slint contact-timing recheck

On 2026-11-13, Slint was built from `6870864f485153e00d5fce2948d73f791ce62802` with locked offline dependencies in Windows MSVC debug mode, using a fresh target directory. The frontend test target ran successfully but contains zero tests. The eight shared GUI-model tests were separately passed on this source in the [GPUI recheck](WINDOWS-GPUI-CONTACT-TIMING.md). Executable SHA-256: `a50ae62660fce7c3937dc3132474a20d55a888fc30ed1ecc2f789e692954aff8`.

The foreground scripted run exited 0 with empty stderr. Its 13 [telemetry records](evidence/windows-slint-contact-timing-20261113.jsonl) match the contact-timing model reference line-for-line through finished tick 1440, and it produced six app-owned software-rendered component PNGs. The inspected tick 480 frame shows the paused status, score 0:1, court, paddles, ball and Slint attribution. This component image is not a desktop screenshot or proof of OS input. An earlier attempt exited 101 because the capture directory had been pre-created; the successful run used a new path as required.

Normal native interaction, physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Previous binaries and captures remain historical.
