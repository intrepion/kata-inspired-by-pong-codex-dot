# Blue Engine transition-frame investigation — macOS, 2026-11-13

The [Mac court-hint check](BLUE-COURT-HINTS-MAC-20261113.md) observed one capture with the window title already at CPU WINS 3–7 while court glyphs and tally were partially stale; a later capture was correct. Slow accessibility/screenshot calls and one sample prevent a reliable frame-by-frame reproduction or attribution to the application rather than observation timing.

A bounded source review found a plausible engine-level ordering mechanism, not a confirmed diagnosis. In cached `blue_engine_core 0.10.0`:

- `src/window.rs:154–177` calls `pre_render`, then updates changed objects, then runs the application callback, and finally submits/presents the already encoded frame.
- `src/render.rs:342–388` records draw calls in `pre_render` using each object's current `is_visible` and its existing pipeline buffers.
- `src/objects/updating.rs:6–12` rebuilds buffers for a changed object after that encoding; `src/objects/transformation.rs:183–185` changes visibility directly.
- The Pong adapter's `rect` changes visibility and, for visible rectangles, changes their size and position in the callback. Its title is also updated in that callback.

Consequently, a subsequent encoding can select visibility established by the previous callback while still encoding the prior buffer objects, because their replacements are built after encoding. That could produce one frame with a mixed visibility/geometry generation, and the separately updated title can lead the displayed frame. This follows from the inspected order but was not isolated as the cause of the observed screenshot; compositor/capture timing and other causes are not ruled out.

A future red-capable check should capture consecutive *presented* frames around a controlled state transition, with a unique old/new pattern and a synchronized frame identifier. An app-level option to test is updating only changed dynamic object buffers at the end of the callback, so the next `pre_render` sees visibility and buffers from the same generation. `Object::update` is public but its `changed` flag is crate-private; indiscriminately rebuilding every object could be costly. Benchmark and verify windowed and headless capture paths before adopting any change. No production fix, security or dependency configuration change, or new package was made.

The reviewed adapter blob is `5cc550a49b29d8c3c21cefc814480eb73660a773`, checkpoint `0988624dcd2a105f612d3ebeaf12b11da7e3be64`. Cached engine source SHA-256: window.rs `acdf710779d0a7e40c06eec9cfbba9268b92749764e47ae2e250a360eec9a7f5`; render.rs `99d20dc06a2a0e64a5d5b160c58e88504a463727c7720bc4926bcbbd2c648260`; objects/updating.rs `bb27e80a866ab03e74f758094df8dd131ff086018eec51e4f02841598d1995d5`; objects/transformation.rs `4d88ea65896db125f0916ab43b0d7c56fcb530d7b4a24d0b53ff02fb969e463c`. The earlier candidate and evidence remain under `/tmp/pong-blue-court-hints-mac-20261113-v1`. No physical-input, license-clearance or release qualification is implied.
