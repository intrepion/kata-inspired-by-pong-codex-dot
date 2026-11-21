# Pong-specific bracket-embedding 0.8.7
The published crates.io archive has SHA-256
`c1cb6747f25903066109a619e3d580ef5ff7b3ffc281fa51ece473e1396e2298`
and records revision `15ec23aba4ec0c147f9ec54d2429a59c1bb6319c`.
The MIT LICENSE is the unchanged workspace-root source notice at that revision.
Only two unconditional PNG constants and the corresponding dictionary inserts
are removed in src/embedding.rs. Both original PNGs are deliberately omitted.
All other archived source inputs are unchanged. This fork supplies no built-in
fonts: downstream graphical applications register their own before building.
