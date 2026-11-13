# Unfocused key actions in windowed Rust adapters — Omarchy, 2026-11-13

Pixels, raw wgpu, ash, softbuffer and ggez paused on focus loss but could still process a later queued keydown before focus returned. The window handlers now track focus, initialize it from winit's window focus state, and reject live keydowns while unfocused. Focus loss still clears movement and pauses; focus gain permits fresh actions. Scripted and benchmark modes remain separate. ggez still accepts keyup to clear held state safely.

In each adapter a regression for a paused game receiving queued Space and R while unfocused failed before and passed after the change; scores remain intact and Space works after focus returns. Full `cargo test --offline --locked` suites passed on Omarchy: Pixels 11, wgpu 10, ash 12, softbuffer 10, ggez 9. Tests exercise the keyboard-action boundary, not actual OS event delivery or physical keys. Rebuilt release packages, native interaction and other platforms remain unverified; Piston, Miniquad and Fyrox remain outside this batch.

Source Git blobs: Pixels `5031e3b3e01cc3e71817507852f8f25ae9963c84`, wgpu `56b5eadfe0996aeb0f17be2b4c269b611c98bd2d`, ash `4fa909f61c31e0036e3da0b267c287c4257a8bc4`, softbuffer `c46cb48675475fb145752446e58a740e8d78b3b0`, ggez `a536df84c04622fd42c567067fda6900f80dae4e`. Existing binaries and captures remain historical.
