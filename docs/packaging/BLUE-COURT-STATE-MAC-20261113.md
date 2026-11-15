# Blue Engine court states: Mac native check (2026-11-13)

The Blue Engine source tested is Git blob `fe4c4783f354fe5020437cc35785855b51d02465` from checkpoint `64f7f16e01e3a13f6c63463e2c59993b80c5547d`, SHA-256 `53aeb2800d33eeee4434d67ce9a4fdab922658a3439ba9879da493da16c30a86`. An offline locked cached release build produced executable SHA-256 `afccfbc11343885e4b402c77ce3140c04099d5e519217edabd3839ee192f4df2`. `cargo test --offline --locked --release --manifest-path experiments/blue-engine/Cargo.toml` passed all 10 tests. Cargo warned that `block v0.1.6` contains code a future Rust version will reject.

In a native Mac window, desktop automation observed the READY panel and 0–0 title, sent Return, then Space; the PAUSED panel and 2–1 title were visible. After Space resumed play, the panel cleared. The unattended match reached 3–7 and the court and title visibly showed CPU WINS. Automated R restarted to Playing at 0–0. The app bundle and prior binary were preserved under `/tmp/pong-blue-court-mac-20261113-v1/`.

This is a native window visual check with automated keys and an unattended match, not physical keyboard or paddle interaction. A player win, resizing, other platforms, performance impact, and clean-machine release/license qualification remain unverified.
