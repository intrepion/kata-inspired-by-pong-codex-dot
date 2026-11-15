# Slint and GPUI shared GUI pause-fix package pilots

On 2026-11-13 Omarchy ARM64 rebuilt Slint and GPUI in release mode from `ccd37831e869cddfc2b37638a4cacdae23ba05cd`, containing shared GUI pause fix `d3fe508f90d840f6be98c88afb17da096acfefef`. Locked offline builds and ten shared model tests passed; GPUI's one geometry test passed, while Slint had no adapter tests. GPUI repeated its existing `proc-macro-error2` future-incompatibility warning. Prior binaries were preserved in `/tmp/pong-gui-pre-pause-20261113/`.

| Adapter | Binary SHA-256 | Local archive SHA-256 | Size |
| --- | --- | --- | ---: |
| Slint | `e3d31bd42437ea0f6c65b6e9981502e219db577f819cf05f1aada510d6664854` | `8a82ab9f99fadea80a577fa0a4a3351f80d1e36bacfa6eb1fde62ee1b83953b7` | 8,875,399 bytes |
| GPUI | `4e8735bb19ff32fe822d3f5357d088da30685b5cf16625fae82dcc87353c51d8` | `05e6a630e4db7ef88480bbc13b029bc8d425e9ba61442f9808e89572ba89096b` | 7,826,699 bytes |

Both archive checksums and all five extracted manifest entries per package passed. With permitted display access, both extracted binaries ran separately from `/tmp`, exited 0, and produced 13 telemetry records matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Slint used `SLINT_BACKEND=winit-software` and produced six component PNGs; tick 480 was inspected and shows paused status, score, court, paddles and ball. These are app-owned component snapshots, not compositor screenshots. GPUI replay is state-only; pixel export remains unsupported and no visual result is claimed.

The pause regression is covered by shared-model tests; these scripted replays do not send native keys or validate toolkit event delivery. Archives, extractions, Slint captures and outputs remain under `/tmp/pong-slint-gui-pause-*20261113*` and `/tmp/pong-gpui-gui-pause-*20261113*`. These local packages do not establish physical input, clean-machine installation, complete notices, legal clearance or release qualification. Earlier evidence remains historical.
