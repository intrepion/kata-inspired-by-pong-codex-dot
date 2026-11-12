# bracket-lib divider contrast correction

2026-11-12, ARM64 Mac, based on `29c16c7`.

The CP437 divider used black cell backgrounds against the dark-blue court, leaving
black boxes around low-contrast glyphs. Divider cell backgrounds now reuse the
court color, and foreground RGB is 120/142/166 rather than 65/81/101.
A regression test requires at least 3:1 relative-luminance contrast for the
non-text divider. Gameplay, sizing and callback timing logic are unchanged.

[Before](evidence/bracket-contrast/before.png) / [after](evidence/bracket-contrast/after.png)
show native tick-600 captures. The corrected divider is visible without the black
boxes; score, paddles and ball remain visible. Nine Rust tests, strict Clippy and
release build passed. Existing upstream block 0.1.6 future-compatibility warning
remains. Native replay exited zero in 13.391 seconds with thirteen states through
finished tick 1440 and six PNGs. No real-input pass inferred.

New binary SHA-256:
`8e6c75ccca7625a83e0c9b93ace64e030cfd5f8be5888bc3e7a024c22458257d`.
Earlier benchmark/package reports retain their original binary hashes and visual
caveat as historical evidence. The new source still needs Windows/Linux visual
requalification and a newly packaged-binary run before those results apply to it.
No signing, license-gap resolution or full release acceptance is claimed.
