# macOS package guard regression checks

2026-11-12:6/6 tests passed on Mac for the Macroquad and SDL3 package creators. Each rejects missing arguments, rejects non-Mach-O input before creating output, and refuses an existing output directory without modifying its sentinel or contents. Tests used the already-built ARM64 pilot binaries; no GUI interaction or system installation.

Run the basic guards:

```
node --test scripts/package-macos.test.cjs
```

On macOS, set `PONG_MACROQUAD_TEST_BINARY` and `PONG_SDL3_TEST_BINARY` to existing thin ARM64 release executable paths to enable both existing-output checks. Those two checks are explicitly skipped without inputs or on other platforms. Each test creates only an isolated temporary directory and removes its own test fixture. Source binaries are read, never modified. These checks do not replace archive extraction, native GUI gameplay, signing, Gatekeeper or clean-machine testing.
