# ggez winner status cross-check on Omarchy

At ggez change `5095cb6d2475bbc4c522784a1514084eeb1a149b`, locked offline `cargo test` passed all 8 tests, including both score winners and clearing the status after restart. A fresh locked debug build passed; executable SHA-256 `b7392ef91c0a71f7fb3a9815fd17a319732f566853984dce3984faed7fcfec8c`, source Git blob `c3ffa2f11e2beb8afb392a2ca5049ad0d8255158`.

The rebuilt debug executable ran `--scripted-test --scripted-captures` on the local display, exit 0 and empty stderr. Its 13 telemetry records exactly matched retained `/tmp/pong-ggez-c77-host-telemetry-20261113.jsonl`; six app-owned PNGs were saved. The inspected tick-480 PNG shows the court, 0–1 score, paddles and readable pause overlay. The scenario does not reach game over, so this is not visual verification of either winner message. Outputs remain at `../pong-ggez-winner-check-omarchy-20261113/`.

The winner claims are unit-level checks of the message selection; rendered winner screens, physical input, full-match interaction, clean-machine packaging and release qualification remain open.
