# Macroquad controls footer: Mac resize check (2026-11-13)

Starting from checkpoint source at `4670c423b775a80b9a3d3754dd1f1835ff96b04c`, the Ready window was resized with desktop automation. At the smaller observed window (screenshot 1008 × 860 pixels including title bar), the controls footer clipped “Esc pause” at the right edge. The updated renderer measures the footer and scales its text to fit the available width, up to its original size.

The updated source SHA-256 was `b5bd3d807d2bc9c582cc615a269ba1d03b0063f149d2a5f955e96a3a5f36dae3`. A cached debug build's executable SHA-256 was `c6d021cb8caabfe6084485a515b7853a880bfcbf967265b34532f483f0e63025`. `cargo test --offline --locked -p pong-macroquad` passed all six tests. In the rebuilt native Mac window, desktop automation resized to the same observed dimensions; the full controls line, including “Esc pause”, was visible, and the Ready prompt remained clear. The original and test app bundles were retained locally.

This is a bounded native visual check at one resized size, not a physical-input test, full gameplay test, or release qualification. Very small window readability and other aspect ratios remain unverified.
