# Local ggez 0.10.0 surface availability patch

Original ggez crate from crates.io; retain upstream licenses. No newer 0.10.1 crate was available when checked 2026-11-11.

The upstream begin_frame loop retries Occluded/Timeout indefinitely. On macOS Metal, a hidden window can remain occluded until the main event loop processes visibility events. The retry loop starves those events and accessibility, consuming a CPU core.

This patch returns without starting a frame for Occluded/Timeout. The default event loop skips draw/end_frame but still executes input-state cleanup. Occlusion detection and error handling remain enabled. Custom event-loop users must check commands() for an active frame, as documented. Lost/Outdated/Validation handling is unchanged.

Two unit regressions cover the classification of deferred vs other errors. A live macOS test verified startup/accessibility, keyboard movement, ball movement, scoring, pause/resume and restart with the patch. This is not Linux or Windows visual validation.

Prefer an upstream corrected release when available. Scoped local vendoring is a temporary workaround; do not edit the global Cargo registry. Review compatibility when upgrading ggez/wgpu. Removing the Cargo patch restores upstream behavior.

The local manifest also adds an empty workspace so direct regression tests remain isolated from the repository workspace.
