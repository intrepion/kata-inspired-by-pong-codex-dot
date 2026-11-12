# Ratatui held-key correction, 2026-11-12

Enhanced terminals report press and release events. The adapter previously stored only one boolean per direction: holding Up and W, then releasing Up, incorrectly stopped movement while W remained held. Down/S had the same flaw. A `w` press followed by `W` release also left movement stuck.

Two new tests reproduced both failures against the previous code. The input state now tracks Up, W, Down and S independently, normalizing upper/lowercase movement characters. Opposite directions still cancel; releasing the last binding stops movement. The fallback terminal's existing latched-direction/X-stop behavior is retained; uppercase X is now also accepted.

## Omarchy verification

- Seven adapter tests pass (five existing, two regression tests).
- Strict Clippy for all adapter targets passes.
- Fresh optimized binary built; SHA-256 `2a2e63ef5abf7cd1ce2ea735983ca546e9d229cf81c7f2a4e1d68e5cdbd10c7b`.
- Scripted cell-export replay exits 0, reports 13 states through tick 1440, and writes six cell-buffer checkpoints. This mode is an in-process renderer/scenario test, not a native enhanced-terminal input test.

No desktop control, OS key injection, terminal screenshot or new package qualification is claimed. Existing archives and their hashes remain historical. Direct key-event regression tests establish the input correction; a real enhanced-terminal keyboard test remains separate.

## Windows follow-up

Source762b301: seven release tests and strict all-target release Clippy pass on Windows. Offline cached test/build used one Cargo job (3.29s/3.06s); no performance measurements or desktop-control retries during concurrent WSL work. Rebuilt executable SHA-256 `475d4c6f9547b60d893e7aa3ff810603ee87ae8090550b90ac9f157014cd9e1f`.

From a new isolated working directory, scripted TestBackend replay exited0 with thirteen records through finished tick1440 and six100x30 JSON cell buffers (3000cells each), plus matching text exports. These are deterministic cell-render/model checks, not real Windows enhanced-terminal input or screenshot evidence. The existing portable package keeps its earlier hash and has not been retroactively upgraded.
