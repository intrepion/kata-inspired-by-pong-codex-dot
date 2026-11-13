# Bevy winner presentation: Windows system test

At checkpoint `3d198e2e55633d104ea5a7282cde6df3645c0c31`, a new Bevy ECS presentation test drives a winning point for each side, runs the actual `present` system, checks its status and score text components, and checks that restart clears the winner and resets the score. It uses a minimal Bevy app without a graphics window.

On Windows stable MSVC Rust, `cargo test --locked --offline -p pong-bevy` passed all seven tests, including the new presentation regression. Cargo reported incremental-cache hard-link failures and copied files instead. The previous test executable was preserved in sibling workspace directory `bevy-presentation-windows-20261113`. The installed stable toolchain lacks rustfmt, so an automated format check was unavailable; `git diff --check` passed.

This is a presentation-system test, not rendered winner-screen evidence, physical input, a complete interactive match, or verification of the separately documented Windows graphics crash. Clean-machine operation, dependency-license clearance and release qualification remain open.
