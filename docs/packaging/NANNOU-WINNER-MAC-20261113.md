# Nannou winner indication

2026-11-13. Nannou showed the score and state in the window title but did not identify the winner in the court at match end. It now displays `YOU WIN - ENTER TO RESTART` or `CPU WINS - ENTER TO RESTART`; a tied Over state has a neutral fallback. The existing pause overlay remains.

On macOS ARM64, offline locked release-profile tests passed 11/11. The new model regression drives a winning point for each side, checks the message, restarts and checks it clears. The cached release build succeeded. Tested main.rs SHA-256 `b4bd27499be24c3ec75fb792770bf441a220f21fbbf40d078d20ed5d9793d61d`; executable SHA-256 `a514522ddeacff3897c3d25d6bca360553333186c459732b5205093c1fc7bdab`. The tested source was based on checkpoint `690c15d80a12cd623725229a2169a2aba2054a62`; Nannou source was checked unchanged against `550905c72d272c625a8ff2db543c4426c1eb442c` before publication.

This verifies message selection and compilation, not the rendered end screen, a full interactive match, physical input or release qualification. Previous artifacts remain intact; the isolated source and target are under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-mac-input-target-20261113-v1`.
