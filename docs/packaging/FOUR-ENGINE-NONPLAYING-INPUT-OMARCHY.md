# Bevy, Blue Engine, bracket-lib and Fyrox non-playing input

2026-11-13, Omarchy ARM64. Focused adapter regressions reproduced the same retained movement in all four: W held or pressed in Ready could contribute movement after play began. The new tests failed before the fixes. They call the adapter input logic, not physical or operating-system key delivery.

Bracket-lib and Fyrox now suppress movement presses made outside Playing until release. Bevy and Blue Engine sample held keys, suppressing each key observed outside play or on the first sampled play frame until release. Their polling gates track whether the preceding sample was Playing and reset that state when a simulation step ends the game; Bevy also resets it on focus loss. The start-frame rule intentionally requires release and a fresh press for a key already down as play begins. Focused regressions cover Ready, Paused and Over and four movement bindings; Bevy also checks first-frame and independent-alias behavior. Polling cannot observe transitions occurring entirely between frames.

Locked offline adapter suites passed: Bevy 4, Blue Engine 6, bracket-lib 12 and Fyrox 5 tests. Separate copied target directories were used for builds. All four release builds succeeded; their scripted runs exited 0 in separate working directories and each produced 13 records matching the shared Rust model. The replay does not exercise the native keyboard path.

| Adapter | Tested source Git blob | Release executable SHA-256 |
| --- | --- | --- |
| Bevy | `99fa17dd490ffb555c29884722141473acbc4e6f` | `7e315116b54fe1f5127812d1870964dd05f4eeaecda0698d9fa7c89d8f4d3cec` |
| Blue Engine | `780318d55144a3711b17a5b693c48005fef930f6` | `643a703843ad4cf6fa70fead689480da55459571ff54b6c78160f52bf56064fd` |
| bracket-lib | `ebb54dfa3e149d926f2883a66e22a94799323d92` | `32c32da696a9d656bcb3503416f22dc69225f9a335a12ed52aa44d3da6ef8821` |
| Fyrox | `d0786475faed0f2e984c3420f8160b6435c25a9d` | `5b80c6bec59fa23fbafafdeac5a2c7650f8c5962676897d59d72558410cf1e74` |

Copied targets, test logs, isolated working directories and replay logs remain under `/home/intrepion/Documents/Codex/2026-11-12/task/pong-four-input-20261113-aAdmKi`. Replay processes ran concurrently; no timing or performance inference is made. Mac/Windows rebuilds, native and physical interaction, complete notices, clean-machine operation and release qualification remain open. Prior binaries and artifacts were retained.
