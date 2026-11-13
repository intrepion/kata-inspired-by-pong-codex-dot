# Piston indicator backing: Mac native check (2026-11-13)

Piston source blob `3ff785fcaa2a5986c7b02243e4d2dad1b9653783` from checkpoint `58ebff2f28`, SHA-256 `5bea945c6252b87e8b36ab80b441a095971dce92d55dcb955ebd03e96f450f31`, was built offline with cached dependencies. The release executable SHA-256 was `c090965d0365fc33707b423fee6b9e01a90ddd3c4770b439bd3fb24ff2f1cb4b`. All 16 tests passed with `cargo test --offline --locked --release --manifest-path experiments/piston/Cargo.toml`, including backing geometry.

The native Mac window visibly showed the Ready play symbol on its opaque backing, separate from the net and ball. Desktop automation sent Return and R, including after clicking the court and title bar; the title and image remained Ready. The reason is not established: event delivery, focus, and application behavior have not been independently distinguished. Consequently this run did not verify the native Paused or winner screens. The app bundle and prior binary remain under `/tmp/pong-piston-backing-native-mac-20261113-v1/`.

This was a bounded desktop-automation visual check, not physical input, a gameplay match, or release qualification. Omarchy's separate scripted paused capture and test evidence is in `PISTON-INDICATOR-BACKING-OMARCHY.md`.
