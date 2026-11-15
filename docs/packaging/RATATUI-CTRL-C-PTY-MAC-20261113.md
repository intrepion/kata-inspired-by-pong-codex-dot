# Ratatui Ctrl-C terminal restoration: Mac PTY (2026-11-13)

The existing Mac release executable from `RATATUI-FOCUS-ACTIONS-PTY-MAC-20261113.md`, SHA-256 `f9b7c4d111dbfae46ea6c97b30d43d3a5dba05851a64eecaed9a13534cb5c3fa`, was reused; no binary was rebuilt. The updated harness from `07d3f38be2` has SHA-256 `d0ff07c2a11d55b166acaba83d6722d7ca8cd73e25de85530b4e5357c65ff875`.

Enhanced and fallback child-process PTY runs entered Playing and injected their respective Ctrl-C terminal encodings. Both exited 0, restored the original slave PTY terminal attributes, and emitted alternate-screen leave and cursor-show sequences. Raw output SHA-256: enhanced `821ab7bcd59d6a3df75373f262d0d091c1233f16e6c84b1061b3074a281d4c04`; fallback `618cba68898c0c455d2991154a45719af643d18d42881e05d0dbb7fa7a1efeef`. Harness and streams remain under `/tmp/pong-ratatui-ctrlc-mac-20261113-v1/`.

These tests inject terminal bytes into a real child process and verify emitted cleanup sequences, not physical keyboard input, signal delivery, forced termination, or actual rendering by a terminal emulator. Clean-machine operation, notices and release qualification remain open.
