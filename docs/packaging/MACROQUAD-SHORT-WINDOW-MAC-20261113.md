# Macroquad short window: Mac visual check (2026-11-13)

Starting with source at checkpoint `18accc957fcde805cda2042ec25dd593307a4964`, desktop automation resized the native Ready window to a short, wide shape (screenshot 1506 × 360 pixels including title bar). The court overlapped the controls footer. A tall, narrow window (608 × 1260 screenshot pixels) kept the footer visible.

The change scales header and footer spacing and text with short window height, and removes the minimum court scale that forced overlap. The rebuilt native window at the same short size showed the court and controls separated. The court and Ready text are very small at that height; this check does not establish comfortable playability there.

Updated source SHA-256: `395db124ee8ca9ada58a02fa5f4084903084b86905ea54e727ef8846c880ea77`. Cached debug executable SHA-256: `06d452031744c5c8f0085dde5bb5428bb4990df48754ed0239c4ddeb706d58f5`. `cargo test --offline --locked -p pong-macroquad` passed all six tests. Both original and new test app bundles were retained locally.

This was a bounded desktop-automation visual check of Ready, not physical input, full gameplay, a scripted replay, or release qualification. Other extreme sizes and the scripted demo overlay were not verified.
