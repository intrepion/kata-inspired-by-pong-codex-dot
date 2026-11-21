# Windows native release input profiles

This opt-in entrypoint prepares separate Windows x64 MSVC build inputs for SDL3 and raylib. It leaves normal development, macOS/Linux and other Windows builds unchanged. The output is a private scratch workspace, not a release archive and not a license or runtime qualification.

Run from an independently reviewed clean checkout; supply its full commit and tree explicitly from the approved release ref. Obtain each original locked .crate through supported Cargo tooling. Do not change entries in Cargo's registry cache. For native compilation prepare an isolated offline CARGO_HOME with the locked dependencies already present, then explicitly select it; the normal user cache is not a build target. Requires the pinned installed Rust target/compiler in inputs-manifest.json, existing MSVC and CMake, and Python with tarfile's data filter. Raylib additionally needs the already installed clang.exe, resource headers and sibling libclang.dll.

    python -B scripts/release/windows/native_release.py sdl3 REPO SDL_CRATE FRESH_SCRATCH --commit REVIEWED_COMMIT --tree REVIEWED_TREE
    python -B scripts/release/windows/native_release.py raylib REPO RAYLIB_CRATE FRESH_SCRATCH --commit REVIEWED_COMMIT --tree REVIEWED_TREE

Add --build --cargo-home ISOLATED_OFFLINE_CACHE for each native build. Add --clang PATH_TO_CLANG_EXE for raylib. Use a short fresh local path without spaces. Failure cleans only the scratch directory this invocation created. Keep the successful workspace, private linker MAP and build JSON for review; never include these logs in a downloadable archive.

The SDL3 optional forced-activation branch is removed; normal activation, popups and public APIs remain. This profile disables joystick/haptic/HIDAPI since Pong uses keyboard. Windows raylib clipboard image acquisition is disabled with an empty image result; text clipboard and normal game/keyboard paths remain. Other flags remain at the recorded defaults. Rust and MSVC path maps are separate; the latter maps only the patched native inputs, never the profile, PDB or output directories.

Before packaging, independently check the exact same executable's map, native source notices, PE/extracted privacy, gameplay replay and captures, and fresh benchmarks at the final source pin. The proposed script does not certify that a reviewed ref is upstream, install software, create a public ZIP or upload anything.
