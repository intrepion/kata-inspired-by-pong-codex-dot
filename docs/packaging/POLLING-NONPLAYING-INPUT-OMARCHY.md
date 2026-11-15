# Polling adapter non-playing movement on Omarchy

2026-11-13, Omarchy ARM64. Raylib-rs, SDL3 and Macroquad had focused regressions fail before the fix: a key held outside play contributed movement when play resumed or began. Nannou had the same polling-gate logic as raylib; its new adapter tests passed after the change, but a pre-fix test result was not retained. These are adapter-level tests, not native or physical key delivery.

The four adapters now suppress each key held during non-playing states or the first sampled frame of play until its release; a fresh press during play moves again. The tests cover Ready, Paused and Over, each of four movement bindings, same-frame start/resume, immediate restart after a game-ending step and independent bindings. Macroquad additionally tests focus-loss suppression. Polling can only observe frame samples, so transitions entirely between samples are outside this evidence. The start-frame rule deliberately requires release and a fresh press for a key already down when play begins.

Locked offline tests passed: Nannou 8, raylib-rs 8, SDL3 7, Macroquad 3. All four release builds passed and their bounded scripted runs exited 0 with 13 records matching the current Rust model. These scripted runs do not test real keys. Source and binary identities follow:

| Adapter | Source Git blob | Release executable SHA-256 |
| --- | --- | --- |
| Nannou | `5fc380a761ddc78a8f478487e31571c575d0e3d0` | `4cf9359e012a5757b4785daf502f6d732ae2ad29f7cbfe43bbbcddf82b1f4025` |
| raylib-rs | `dbad44c0666c2f533e27e180d04e829e6fdbd641` | `8712b5729662cd7919e63237dbae6b963f462ebd342a024e41da1324a8ff8cf3` |
| SDL3 | `e5fcb679e49e4b7600c4a9d7082f448ee691b549` | `1599cccf1cf60ca3082584aba4c49aab64dff41dc3546cafee57140fcf41b636` |
| Macroquad | `80b9fb91ad84c313bf45b0d2f803c94987d8bec9` | `59d391c1e97e2b5efce1e526bd1482594cbb638e79d0ce91f798bcd7c2525372` |

The [six previously fixed event adapters](NATIVE-NONPLAYING-INPUT-OMARCHY.md) were also run in bounded scripted replay: Miniquad, Pixels, wgpu, Piston, ash and softbuffer all exited 0 and each produced 13 model-matching records. These runs were concurrent and are not performance measurements or native-input verification. Fixture targets and logs remain under `/home/intrepion/Documents/Codex/2026-11-12/task/pong-polling-nonplaying-20261113-f08dxB` and `/home/intrepion/Documents/Codex/2026-11-12/task/pong-native-nonplaying-20261113-1B4ji0`.

This is not a complete native adapter audit: Bevy and Blue Engine also poll held keys and need separate regression checks, as do other independent input paths such as bracket-lib and Fyrox. Actual key delivery, physical input, Mac/Windows verification, clean-machine and release qualification remain open. Prior evidence and binaries were preserved.
