# Windows corrected GUI/terminal input package follow-up

New local ZIPs and fresh extraction paths containing spaces were created for corrected Ratatui, Slint and GPUI release binaries. Each executable hash was required to match its prior corrected-input native evidence before packaging, then checked again after extraction. [Exact hashes and results](windows-gui-held-package-20261112.json). Old archives and all new artifacts remain retained.

| Adapter | Extracted result | Evidence limits |
| --- | --- | --- |
| Ratatui | Exit 0, thirteen states, finished tick 1440; six JSON buffers of 100x30 cells and six text exports | TestBackend cells, not a Windows terminal screenshot/input or terminal restoration pass |
| Slint | Exit 0, thirteen states, finished tick 1440; six own-window PNGs | Tick 600 inspected, readable score/controls/court and required Slint attribution preserved; not compositor or keyboard acceptance |
| GPUI | Normal-window launch exits 0, thirteen states, finished tick 1440 | State-only, no app-owned capture API; initial hidden launch failed as below |

## GPUI hidden-launch limitation

The first extracted GPUI run used the Hidden launch option and exceeded the 45-second deadline. A further bounded 30-second observation still did not finish; only tick 0 appeared and no main window was reported. The responsive process was stopped after its exact owned executable path was checked. Its logs remain retained. A single controlled diagnostic using the same executable, same working directory and scripted argument but Normal window style completed successfully. This is a qualified normal-window pass, not a hidden-launch pass, and no source or security settings were changed. The observation suggests launch visibility is relevant; it does not establish a general root cause or fix.

These are local developer-machine portable runtime checks only. Archives contain the executable, project LICENSE and an explicit incomplete-pilot warning. Full notices, Slint licensing selection, clean-machine dependencies, signing, installation/upgrade/uninstall and public distribution are not qualified. No development artifacts were removed, and no physical keyboard/focus pass is inferred from scripted input.
