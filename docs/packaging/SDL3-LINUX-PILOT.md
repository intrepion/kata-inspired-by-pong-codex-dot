# SDL3 Linux ARM64 portable-package pilot

Verified 2026-11-12. Application source is checkpoint `40c40f246e134659a48023ba5dd9d079a3404596`; packaging helper is introduced alongside this report. This is a local lifecycle experiment, not a release or a system installer.

## Passed

- `bash -n` validates the app-owned packaging script.
- New-output-only archive creation; second invocation refuses the existing output directory without overwriting it.
- Archive SHA-256 and all extracted-file checksums verified.
- Extracted binary ran from an isolated directory outside the source checkout. The bounded app-owned replay exited 0 with 13 records and finished tick1440.
- Uninstall removed only the five verified package files; `rmdir` removed the now-empty package directory. A neighboring sentinel and original development binary remained unchanged.

The first restricted launch returned `No available video device` and emitted no replay records. A supported, approved normal-display launch then passed. No OS input injection, desktop screen capture, settings change or desktop-control bypass was used. The replay is application-owned stimulus and state evidence, not physical-input or visual-quality verification.

## Artifact provenance

- Binary SHA-256: `2c18942de6257c087d7002b3d0c88fd5ac616a03b8a25f437e236abc469a77a9`
- Local tar.gz size: **1,999,847 bytes**
- Local tar.gz SHA-256: `5b9cf9e82ed35cf71b731b0caa859f826c63acffe7a9832d3d5f79d51bbec31a`
- Archive is retained locally, not uploaded as a release/download.
- Packaging metadata intentionally excludes machine identity, environment variables and filesystem paths. The explicit source commit parameter is supplied by the operator; the pilot independently matched the measured binary hash.

## Remaining release gates

Complete dependency license/notice inventory, clean-machine library testing, upgrades, signing, real installation UX and compatibility with other distributions remain unverified. SDL3 is statically included by the default feature, but the application still needs compatible Linux/glibc and platform graphics/audio libraries. This is not a fully static binary, Flatpak, AppImage, deb or rpm.

Run the helper alongside the SDL3 client:

```
bash experiments/sdl3/packaging/linux-portable.sh RELEASE_BINARY NEW_OUTPUT_DIRECTORY SOURCE_COMMIT
```

The shared gameplay core has no packaging responsibilities. This second app-local helper deliberately remains small until cross-app requirements justify a shared utility.
