# Ratatui Linux terminal package pilot

2026-11-12, Omarchy ARM64. Fresh offline locked release build from game-source checkpoint ed2ef73; five frontend tests pass. The shared portable packager now includes a terminal-specific guide: interactive terminal, minimum 50×18 cells, and --scripted-cells rather than PNG capture. No graphical desktop requirement is claimed.

## Verified

- Fresh 413,368-byte archive, SHA-256 manifest and extracted file checks.
- Relocated deterministic TestBackend replay: exit 0, 13 states through tick 1440, finished=true.
- Six cell checkpoints, each JSON plus plain text. [Tick-600 text](ratatui-linux-cells-tick0600.txt) inspected; includes score, fallback control instructions and court. These are terminal cells, **not screenshots**.
- Same binary launched normally in an isolated pseudo-terminal. Fallback input label and Ready state appeared; Space produced changing game output; Q exited normally. Exact stty flags before/after matched, and alternate-screen/cursor restoration sequences appeared.
- Additional S, X and Space bytes were sent, but ANSI-diff output did not independently establish their complete effects. No new pause/held-key/full-match pass is inferred.
- Exact five-file uninstall preserved neighboring sentinels and the original release build.
- 28 shared Node benchmark/packaging tests pass, including terminal guide expectations.

[Archive and cell digests](ratatui-linux-package-20261112.json). The first archive's generic graphics-requirement wording was corrected; the final archive was regenerated and rechecked with the unchanged binary. Both temporary installations were removed, archives/evidence retained.

## Limits

PTY output and TestBackend cells do not validate a desktop terminal emulator's font, enhanced key-release reporting, focus events or actual keyboard input. No Linux desktop control or global input injection was used. Full dependency notices, public distribution, clean-machine/cross-distro installation and upgrade support remain unqualified.
