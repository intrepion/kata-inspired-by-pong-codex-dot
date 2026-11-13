# Windows polling active-restart recheck

At checkpoint `dafe038b46827f409d711b14ea733b4d38d4b081`, including `39d3037` and `3586d99`, Windows MSVC stable Rust 1.99.1 passed locked offline debug tests for Nannou (10) and Macroquad (5), including their active-restart held-input regressions. Tested source Git blobs were `ee0bc8d5bbc8dd661de9354d147867dbed843207` and `c7f07f218656d7da29b7dc21cccb2c1983a3d40b`, respectively. Cargo fell back to copying incremental cache files when hard-linking failed.

A fresh Macroquad debug executable built with locked offline dependencies had SHA-256 `ecf48bc4746a482ccc8174d522e581d6b51cb1b94fee651489db0c939274dc0c`. Its isolated scripted run exited 0, stderr was empty, and all 13 records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. No capture was requested. The copied target is `target-polling-active-windows-20261113`, and replay logs remain in sibling workspace directory `polling-active-windows-20261113`.

A separate Fyrox target copy was stopped as free disk space fell; the partial copy and original remain intact. No Fyrox result is claimed for this revision. Raylib was not rerun after its stalled native build. These tests call adapter logic; the standard replay does not verify held physical keys on restart, device accessibility, clean-machine operation or release qualification.
