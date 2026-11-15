# Ratatui focus-action gate: Mac PTY cross-check (2026-11-13)

Ratatui source blob `79ed6747219ee0e4c8b8a3e9baf2523043e66871` from `ddb09af6c6`, SHA-256 `5794412c12ec06c8b1fbdf0edbfe4b71bdee6b9f632f55fb871c529bbc4e146a`, was built offline and locked with the cached Mac release target. Executable SHA-256: `f9b7c4d111dbfae46ea6c97b30d43d3a5dba05851a64eecaed9a13534cb5c3fa`. All 11 release tests passed.

The project PTY harness, SHA-256 `a9227f0b6b7fab364318c5690ee73eefe2c4bab35886a00054216fd1c4cb2bf7`, passed both enhanced-key and fallback runs against the release binary. Injected focus loss kept Space, R and Q from acting until injected focus gain; Space then resumed, Q exited, and terminal attributes were restored. The enhanced run also exercised movement suppression and resize; the fallback run checked latched movement and explicit stop. Raw output SHA-256: enhanced `49e1bf1595d27d8e077c69a0febd72dfb09d247d9da7a6d72b95f6b2d9408b6f`; fallback `6d896d28e80c316414a48c042b6fa7afb0056b23a7ed6f5b4ff4606dd560198e`. The previous binary, harness and raw output remain under `/tmp/pong-ratatui-focus-gate-mac-20261113-v1/`.

These are real child-process PTY tests with injected terminal protocol bytes and a limited output parser, not physical input or a real terminal-emulator focus transition. Other terminal compatibility, clean-machine operation and release qualification remain open.
