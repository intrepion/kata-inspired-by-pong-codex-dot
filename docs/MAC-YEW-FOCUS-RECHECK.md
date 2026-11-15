# Yew Mac focus follow-up: inconclusive

2026-11-12 UTC. Used the previously fresh native bundle with SHA-256 `bd4323bdc07be4ffcda605a2e44aaf0a39ea8186906bfc3d61f82c467038eddb`, matching the `b92b7df` build recorded in [native follow-up](MAC-NATIVE-FOLLOWUP-20261112.md). Not a current-source rebuild.

Supported native-app controls started play. A combined Cmd+M and Finder-menu interaction sequence was followed by a visible Paused state at1:1. In a second attempt, explicitly resumed play then opened Finder's About dialog (the tool reported that dialog focused); Yew subsequently remained Playing at2:2. Test app and test-created dialog were closed afterward; a separate attempted CLI instance was stopped.

**No general OS focus-loss pass or definitive new bug claimed.** App-targeted controls and focused accessibility elements do not independently establish the native key-window/deactivation event delivered to Yew. The mixed observations therefore do not resolve the existing lifecycle gap. Source still needs a proper native/window event check or an independently verified real switch. No security bypass or undocumented desktop injection was attempted. Other verified gameplay results remain unchanged.
