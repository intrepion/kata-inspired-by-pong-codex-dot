# Godot Mac contact-timing attempt

macOS ARM64 source `2582b78af9ffecb254c4367184cad76640999e31`, including physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. An offline locked debug GDExtension build reused a copy-on-write cache; a handful of small dependencies and local Pong crates compiled in 10.49 seconds. Extension SHA-256 `cbd5fa8f5c7d848fa68187bf5bb77bc508c01c80b645affec8e1b80e9a3d82b6`.

A separate project copy and extension-list file were created for a sandboxed Godot 4.5.1 headless smoke run. It failed to open `user://logs` and crashed in `RotatedFileLogger` before assertions (exit -6). This is an environment-limited failed attempt, not a passing integration or gameplay test; no editor-shutdown conclusion follows. Project and build artifacts are preserved. A suitable writable user-data context and current-physics runtime checks remain open.
