# Shared GUI explicit-pause held-input fix

GTK4, Slint and GPUI use `pong-gui-support::Model`. Before this change, Space pause left movement bindings active, so a held direction could move the paddle again immediately after resume. A new model test reproduced the stale input before the fix. Unlike the Miniquad adapter, some GUI event paths do not pass a repeat flag to this model.

On entry to Space pause, the model now suppresses each held movement binding until its release. Its held-key set remains intact to continue suppressing repeated Space and movement key-down events; releases remove suppression, and an independent fresh binding can move normally. Focus loss clears both held and suppressed state. The ten release-mode shared model tests pass on Omarchy, including both directions, aliases, case-changed repeats, fresh presses and blur. Locked offline `cargo check` passed for GTK4, Slint and GPUI; GPUI repeated its existing `proc-macro-error2` future-incompatibility warning.

These are model and compilation checks, not proof of toolkit-specific OS event delivery or physical interaction. Toolkit event streams that synthesize release/press pairs were not tested. Rebuilt client binaries, actual pause input, scripted captures, other-platform checks, package requalification and release qualification remain pending. Previous adapter/package evidence is historical; shared physics was not changed.

## Mac Slint recheck

macOS ARM64 source `59957bbdd5f7dd9e61fa1367d522e809b557df62` includes this fix. Offline locked debug build reused a copy-on-write cache; all ten shared GUI support tests passed, including held-input suppression and repeat handling. Slint executable SHA-256 `92e71de96fb57b7f8ba6044d46fd3c9372d3c35a98e69ec540d6e50bb11bc80b`.

A locally assembled test app opened through the supported desktop interface. Sequential automated Enter, held W, Space, Space, held S and Space produced visible upward movement, Paused, resumed play and downward movement, then Paused. The app exited. W was released before the first pause: the held-across-pause sequence is tested at the shared controller boundary, not independently demonstrated by native desktop automation or physical input. GPUI and GTK native interaction were not tested in this run.
