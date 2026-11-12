Pong SDL3: Linux portable-package lifecycle pilot

Extract into a NEW directory and run ./pong-sdl3. Space starts/pauses/resumes;
W/S or arrows move, R restarts, Escape exits.
Bounded app-owned replay: ./pong-sdl3 --scripted-test
Verify package files: sha256sum -c SHA256SUMS

No root, package-manager install, desktop entry or PATH modification is needed.
Uninstall only the package-owned extracted files; retain any personal files.

Requires matching architecture and compatible Linux/glibc, graphics libraries
and drivers. The default build includes SDL3 statically but is NOT fully static.
This pilot is not an AppImage, Flatpak, deb/rpm or cross-distro compatibility test.

Root MIT license included. Full dependency-license inventory, signing, clean
machine testing, upgrades and release packaging remain outstanding.
Do not distribute this internal pilot archive as a finished release.
