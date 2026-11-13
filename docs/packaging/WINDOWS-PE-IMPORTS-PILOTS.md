# Windows PE import check for three portable pilots

On 2026-11-13, MSVC `dumpbin /dependents` from Build Tools 14.44.35207 was run against the extracted, hash-verified [ggez release](GGEZ-HELD-WINDOWS-RELEASE-PILOT.md), [Nannou release](NANNOU-HELD-WINDOWS-RELEASE-PILOT.md), and [raylib debug](RAYLIB-HELD-WINDOWS-DEBUG-NOTICE-PILOT.md) pilot executables. All three commands exited 0 with empty stderr. No binaries were rebuilt or launched for this inspection.

| Pilot | Executable SHA-256 | `dumpbin` output SHA-256 |
| --- | --- | --- |
| ggez release | `3bf85d385fc754154480d9da73a8486ce8d9efc70e4b7fbd24e10609fcf1ce35` | `04fd828054e379da80ea3e94558f517064c72e25b607b436f79f0bdd8521ffd0` |
| Nannou release | `4b590de541004ee31f7989be554c64a5685ec26c67abbdd1368b8485fbd24624` | `9a97668a18190d1eb4d8f08a81b906c33d4716badf3e1d0b6c42c13e47f67c13` |
| raylib debug | `ba42e6c923376f394e74d8f4fbb809afd3d2a13e38903b9b977c1cfd3dd8ecd1` | `1124e852bf9e3c07a63d7f4e8c2d74b921e93984546c942e2030d3f163fbeff5` |

Each PE dependency table names `VCRUNTIME140.dll`, Windows API-set CRT libraries, and Windows libraries including `KERNEL32`, `USER32` and `GDI32`. The ggez and Nannou binaries also name `opengl32.dll`; Nannou names `dwrite.dll` and `uiautomationcore.dll`. Raylib additionally names `WINMM.dll`. None of these pilot packages includes `VCRUNTIME140.dll`. This host has a System32 copy reporting file version `14.50.35719.0`; its mere presence does not prove the loader chose that copy or that another machine has a compatible runtime. The raylib binary's direct imports do not resolve the separately documented debug-build LNK4098 warning.

The full command outputs and empty stderr files are retained under task workspace `windows-pe-imports-pong-20261113`; output hashes above include local binary paths and those raw files are not published. PE import names are not a complete dependency inventory: runtime-loaded graphics drivers and libraries, delay loading, transitive dependencies, actual DLL resolution, prerequisites, clean-machine execution, security updates, licensing and redistribution remain to be checked. Earlier successful same-host scripted runs and package hashes retain their original scope. No installer, runtime installation, system change or removal was attempted.
