# Windowed unfocused-action cross-check on macOS

2026-11-13. The source for the focus-loss keydown fixes at `41f99b347722f5989aba304778eefd08272fdd98` was fetched into the isolated Mac source overlay. Offline locked release-profile tests passed on macOS ARM64:

| Component | Passed | main.rs SHA-256 |
| --- | ---: | --- |
| Pixels | 11 | `4f3f070e88dc9a31cd9781162b54788d1814198eda3ba06d46036fb0c5874566` |
| raw wgpu | 10 | `93afa3349b8fbbd96aa045d58370b98adc73c10cfa918b4aaea1decefaec0126` |
| ash | 12 | `e499296a81204d39cac1cb5c33b12d9c7dac2cb00c737ba74b148bc79aa132fe` |
| softbuffer | 10 | `8f4d7f98181198e63a2f78bbdd5a4e6815a3b6f5b7f978439883522f1a102a57` |
| ggez | 9 | `61b8c8d809c8f09c57bc411a68adc90bc55cdc73ef1e723aaa42bc2329159e8e` |

Each suite includes `unfocused_key_actions_do_not_resume_or_restart`. These are adapter/model tests; no native focus transition, physical input, graphical replay, Vulkan runtime or release qualification is claimed. Existing cached targets were reused and prior artifacts preserved.
