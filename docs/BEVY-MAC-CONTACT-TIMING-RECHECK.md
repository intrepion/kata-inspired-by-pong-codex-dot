# Mac Bevy contact timing recheck

macOS ARM64, source checkpoint `173c0ef1ea933479159e561a97028a4aa1118f67`, including physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. An offline locked release build reused a copy-on-write cache and compiled only local workspace crates. Binary SHA-256: `4a9ab53e26603201ffe7dd6dd377dc2a0e0637fbe579cc9a9f8dccf55e93c40f`.

Direct-shell `--scripted-test` exited 101 before gameplay: Bevy reported it could not find a GPU. It is not a replay pass. The packaged app opened through the supported desktop interface and displayed the ready court. Automated Enter started play, a held W moved the left paddle upward, and Space displayed the paused state. Enter resumed; R returned the paddle to center with a 0–0 score visible. The test window was closed. This bounded automated desktop check does not establish a full match, physical input, app-owned captures, signing, or clean-machine release qualification.
