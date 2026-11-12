# Linux held-input-fix package follow-up

Miniquad, pixels, raw wgpu, ash and softbuffer local archives were regenerated from the corrected release binaries. Each input binary SHA-256 was required to match its preceding corrected native-run evidence before packaging. No old binary was relabeled as fixed, and older archives/results remain historical.

Each new archive was extracted into a separate fresh directory, its manifest verified, and its executable run through the bounded native scenario with app-owned captures. Results and exact archive/binary hashes are in [the redacted record](linux-held-fixes-package-20261112.json). Pixel PNGs are CPU-source images, not proof of final window presentation. Other captures remain app-owned, not compositor screenshots. Replayed state does not replace physical-keyboard validation.

After the processes exited, only the four manifest entries plus SHA256SUMS were removed. Empty-directory removal and neighboring sentinel verification detect leftover state rather than recursively erase it. Original binaries, archives, capture directories and private logs remain local.

All sixteen applicable Linux packaging tests pass; two Windows-junction cases are skipped on Linux. This is local extraction/runtime/removal qualification, not clean-distro installation, signing, complete dependency-notice review or public redistribution. Timing evidence is not rerun or superseded by this package check.
