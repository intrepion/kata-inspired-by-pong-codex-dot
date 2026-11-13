# Windows polling focus/resume recheck

At checkpoint `437ae1ceb501b5b7c7a478bbf5043eedce75863a`, including `ad437fd`, Nannou source Git blob `33b455e2a54c3ed4dc71aeb82b8d1a67c381a890`, Windows MSVC stable Rust 1.99.1 passed all 12 locked offline debug tests. The suite includes `focus_loss_then_immediate_resume_suppresses_held_key`, covering a held key, release and fresh press. Cargo fell back to copying incremental files when hard-linking failed. The previous test binary and symbols were copied to sibling workspace directory `nannou-focus-resume-windows-20261113` before reusing the existing target.

These are direct adapter and polling-gate tests; no actual window focus transition, physical key, new executable or replay is claimed. Raylib was not rerun after its stalled Windows native build. Accessibility, clean-machine operation and release qualification remain open.
