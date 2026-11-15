# SDL3 sys macOS libusb backend patch

This candidate vendors the published sdl3-sys 0.7.3+SDL-3.4.20 crate. Its registry archive SHA-256 is `0e68da4d2bcf29a81a1ea71ec27b93bb27151d649c2b186eb5988ad4dcb17d11`, matching the original Cargo.lock. The upstream Zlib license is retained in LICENSE.md.

The only upstream code change is in build.rs: for bundled source builds targeting macOS, define SDL_HIDAPI_LIBUSB=OFF. The native HIDAPI, HIDAPI joystick, and SDL joystick options are not disabled by this patch. Other target OS values retain upstream configuration. This removes an optional backend and may affect devices that require it; individual controllers have not been qualified.

This patch does not guarantee the provenance of binaries supplied to the existing packaging scripts. It does not apply to external/system SDL builds, establish the licenses of every native or system component, or constitute distribution approval. A complete package build and license review remains required.
