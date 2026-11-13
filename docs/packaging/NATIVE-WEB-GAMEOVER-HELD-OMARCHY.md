# Native and web input held through match end

2026-11-13, Omarchy ARM64. Focused adapter regressions reproduced movement carrying into a new match when a direction remained held through a real shared-core winning score. Tests failed before the fixes in Miniquad, Pixels, wgpu, Piston, ggez, ash, softbuffer, Fyrox, bracket-lib and the shared Rust web controller. These tests directly call adapter/controller input functions; they do not inject OS or browser events.

On restart from Over, the native event adapters now clear active movement or suppress still-held bindings until release, preserving repeat handling. The shared web controller clears keyboard and pointer ownership on restart from Over; keyboard repeats cannot rearm it. Regressions exercise four native movement bindings and each adapter's relevant restart controls; the web regression exercises a keyboard binding and a pointer. The scoring fixture awards the player the winning point. These changes address restart from Over; they do not claim to change every manual restart during active play.

Locked offline suites passed: Miniquad 12, Pixels 9, wgpu 8, Piston 11, ggez 6, ash 10, softbuffer 8, Fyrox 6, bracket-lib 13 and web controller 15. The shared web controller also passed locked offline `cargo check` for `wasm32-unknown-unknown`; its tested source Git blob is `6e5d924b562a2ac9ce7523c79266421652a6b7e6`. Rebuilt browser frameworks and desktop wrappers remain unverified for this change.

All nine native release builds succeeded in isolated copied targets. Each ran a bounded scripted replay, exited 0 and produced 13 records matching the current Rust model. The standard replay does not reach game over or exercise real keys. Replays ran in groups using separate working directories; these are not benchmark results. Source and binary identities:

| Adapter | Source Git blob | Release executable SHA-256 |
| --- | --- | --- |
| Miniquad | `27200a6e111a2729cd6eb53bd14f5c6f3ed843ee` | `098e9ec13bbcfb36e2697fcae2475e9bacb92e1c5e0ef3ea490b025fa3084304` |
| Pixels | `9502c2d0d6dd591a2d7319cffa359a1df0a95f2b` | `4a76698cdaf727dd143c1cacc93a3f18fd5ffcfe6e627ea9b087230c92c22375` |
| wgpu | `6328698b25a40496b47f9f0c0303dc59b22c49d0` | `510c2462f8bddd00d3df39fb25d98bb2a0e352c0957840d4ba49cbe1e6ad3975` |
| Piston | `5b8b519151b9604d0be9472bde4f6cd284523970` | `3427a726a4dece3b57fcf2e68609179bfe390a4f0845bdc7d426653042a71c95` |
| ggez | `8c97f68d60cfb239a1bd045c63fe3048be644d4e` | `8005734e07f5fb2d80f8ac52d8d9b2e723a9a91059fbbaadfe62c2456f302d12` |
| ash | `3ca728f2e8973538871d0499c11db54c3ec8c233` | `59c4e0b3a4506d386a4ea629191fe3998e73cbb3c9feda96f8cd9569d10b0fa9` |
| softbuffer | `ee9a7efe8854e2cb229646ec6e132d5d36e6949e` | `f5347829432e5458658f238181c4e27bf670372bd9695d7ea79cf36213fa2f66` |
| Fyrox | `3c650b5f397e8075b105fa7a0989076df910aaa9` | `fd279613745218b140afc5e27d5c9decd995b9eb9b56f4cadeb1824c4430f33b` |
| bracket-lib | `7286ab5f2c14796aab65452db6fc2cf19f62f4d8` | `f3b4d3689b98335d73cb804fe9285d91f6aff9b8e07d9ccef788b5112dc24244` |

Copied targets, isolated working directories and logs remain under `/home/intrepion/Documents/Codex/2026-11-12/task/pong-matchend-input-20261113-ohoNTk`; prior artifacts were retained. Physical/native or browser input, both-winner adapter integration, Mac/Windows rebuilds, dependency/notice review and release qualification remain open. Polling adapters and mobile paths were not requalified by this change.
