# Macroquad Windows x64 portable packaging pilot

2026-11-12 UTC. **Local ZIP creation, extraction, hash verification and extracted executable scripted replay passed. Not a distribution-ready installer.**

Reused the release binary built for the Windows timing pilot (source f832743 plus the documented runner-only patch, before subsequent test-only snapshot changes). Binary SHA-256: `b4ecbfa4901198fb0a49d03ed58ec598175ecde0283e02c5f7c84832c67f75f1`. No physics changes since that source were needed. The exact archive and binary hashes are in [manifest](macroquad-windows-manifest-20261112.json); ZIP size1,419,549 bytes.

`clients/macroquad/packaging/package-windows.cjs` validates x64 PE headers, creates only a NEW output directory, includes the project MIT license and explicit pilot notice, and creates a portable ZIP. Syntax check, rejection of existing output and rejection of invalid PE input passed. It does not install, edit PATH/registry, change permissions or delete anything. The source binary remains untouched.

Archive extracted outside the checkout into a path containing spaces. Extracted binary SHA-256 matches the source exactly. Launched from that extracted directory with `--scripted-test`: exit0, no stderr, no timeout, all13 JSONL checkpoints ordered0..1440; Ready, Paused, resumed play, score/reset and final finished flag checked. [Raw application telemetry](macroquad-windows-scripted-20261112.jsonl). Process was confirmed exited. This tests the application's scripted replay path, not actual keyboard input or complete-match gameplay.

Read-only PE import inspection lists Windows DLLs only: IMM32, KERNEL32, msvcrt, ntdll, USERENV, WS2_32, GDI32, SHELL32, USER32, bcryptprimitives and Windows API-set libraries. This is not exhaustive dynamic-load analysis; OpenGL driver availability and clean-machine compatibility remain unqualified.

Pending: full third-party notices/license audit, clean-machine install, upgrade/uninstall lifecycle, downloaded-file SmartScreen handling, signing and public distribution. No downloaded security warning was bypassed, no files were publicly uploaded, and all staging/build artifacts are retained. Portable extraction is not an OS installer pass.

A second extracted run with `--scripted-captures` also exited 0 without stderr, producing all six requested PNG checkpoints. Inspected [paused frame](macroquad-windows-extracted-paused.png) and [final frame](macroquad-windows-extracted-final.png): court, paddles, ball, readable score/instructions and pause overlay rendered; final scores reset to 0:0. These are app-owned framebuffer captures, not desktop-control screenshots. They do not prove OS input integration. Other four captured checkpoints are retained locally and were not individually inspected in this pass.
