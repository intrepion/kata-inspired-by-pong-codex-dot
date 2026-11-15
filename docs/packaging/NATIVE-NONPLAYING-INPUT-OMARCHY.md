# Native non-playing key presses on Omarchy

2026-11-13, Omarchy ARM64. Following the [ggez fix](GGEZ-NONPLAYING-PRESS-OMARCHY.md), direct adapter tests reproduced retained movement when a fresh movement key was pressed outside play and then play started or resumed in Miniquad, Pixels, raw wgpu, Piston, ash and softbuffer. Each added regression failed on Ready/W before its fix. These are adapter-function results, not native or physical keyboard delivery.

The six event-driven adapters now ignore movement presses outside Playing while continuing to process releases. Per adapter, the regression covers W/Up/S/Down in Ready, Paused and Over, repeated events, release, and fresh presses during play. Locked offline test suites passed: Miniquad 11, Pixels 8, wgpu 7, Piston 10, ash 9 and softbuffer 7. All six locked offline release builds and formatting checks passed using separate copied target directories. No replay or native-input run is claimed for these rebuilt binaries.

| Adapter | Tested source Git blob | Release executable SHA-256 |
| --- | --- | --- |
| Miniquad | `7824569589549c1150e748fbd5db8f7315db99f1` | `e5a94e08e66da95d415d94b305bf35485a7cb6e8edaacfa8ac55c877261cfe75` |
| Pixels | `7654c60a42fd7e41f747dc61d2188612e06eb933` | `5a86d5bc20051835b57eabefab9f0aadf3f6d5af827e84def4e72f861582e32e` |
| wgpu | `873b7b49a46d89c7ed36720495905db0d41bd7e0` | `2d5bb771f4f7a2539452ffb6eb98b94de2a08026cd174c0e466a7673ca5d58f5` |
| Piston | `ef5454dd3d92113a67ae83e0dae135d4233f13ef` | `a1eacf599f4458464d1a0061d028724d868d828b24abd1525d85d31951db2e70` |
| ash | `bf3653ed9db303c7f3f706c5930575b09b733645` | `8bdf3083c59db3854a1ecae169c9765cb9979357dc7803a2462da5ce25a0ba8c` |
| softbuffer | `196e9612946d5576e99c59a7cc4c9e69b993494f` | `19d69e8bf5369996d7f89def98e0493935e74fa8446b99258100720aad2bc5f6` |

The copied targets remain under `/home/intrepion/Documents/Codex/2026-11-12/task/pong-native-nonplaying-20261113-1B4ji0`. Polling-based native adapters have different held-state semantics and are not covered by this change; their non-playing press behavior needs separate investigation. Replays, actual keyboard interaction, Mac/Windows rechecks, clean-machine qualification, notices and release readiness remain open. Existing artifacts were retained.
