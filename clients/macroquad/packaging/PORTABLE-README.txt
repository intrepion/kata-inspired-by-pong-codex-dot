Pong Macroquad: local Linux portable-packaging pilot

Extract into a NEW directory. From the extracted folder run ./pong-macroquad.
Enter starts/resumes; W/S or arrows move; Space pauses; R restarts.
For the bounded deterministic smoke test, run ./pong-macroquad --scripted-test.
Verify contents with: sha256sum -c SHA256SUMS

No root, package manager, desktop entry, PATH change or system install is needed.
To uninstall, remove only this extracted package directory, after checking that
it contains no personal files. The automated pilot removes only its own known
staged files and uses rmdir, which refuses directories containing other files.

This binary requires the recorded architecture, compatible Linux/glibc and
working graphics/display libraries and drivers. BUILD-INFO records direct ELF
links; dynamically loaded graphics libraries are not fully covered by ldd.
This is not AppImage, Flatpak, .deb or .rpm, nor a cross-distro compatibility claim.

Internal validation artifact only. Root project MIT license is included.
Dependency-license notice inventory, signing, clean-machine dependencies,
upgrade behavior and distribution-specific installers remain release gates.
Do not publish this pilot archive as a finished release.
