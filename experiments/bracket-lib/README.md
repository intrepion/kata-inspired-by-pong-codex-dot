# Pong bracket-lib

A **graphical** retro-style Pong frontend using `bracket-lib = 0.8.7` (formerly RLTK), with its default OpenGL backend and bundled CP437 font. This is distinct from `../ratatui`, which runs inside a terminal.

```sh
cargo run --manifest-path experiments/bracket-lib/Cargo.toml
cargo test --manifest-path experiments/bracket-lib/Cargo.toml
```

- Hold W/S or Up/Down to move; releasing stops movement.
- Space starts, pauses or resumes; R restarts; Q or Escape exits.
- Losing focus or resizing pauses and clears held input. Press Space after returning.
- Same shared `pong-core` CPU opponent, scoring, ball physics and fixed 120 Hz simulation. Rendering quantizes to 90×50 square tiles; this affects appearance, not collisions.
- Fixed logical grid scales with the graphical window, rather than changing the playfield dimensions.
- Independent Cargo workspace and lockfile; no change to original web game or root workspace.

## Scope / limitations

This is a learning comparison, not an endorsement of bracket-lib for a new production game. Its pinned release uses older winit/glutin/OpenGL dependencies. Linux may require X11/Wayland development packages; Windows requires a working graphics driver; macOS OpenGL is deprecated. A build on one OS does not establish compatibility on others. Browser/WASM packaging is outside this native frontend pass.

Tests cover held/released input, repeated pause-key events, focus cleanup, bounded catch-up and world-to-tile clipping. Interactive gameplay verification is separate from these tests.

Sources: https://github.com/amethyst/bracket-lib and the pinned crates.io source/API.

## Verification in this pass

macOS arm64: all four frontend tests pass; debug build succeeds; native window initializes OpenGL 4.1 Metal. Computer Use inspection verified ready rendering, starting, held-W paddle movement, ball/CPU movement and pausing. A discovered center-line glyph overlap was fixed and the corrected ball rendering rechecked. Restart/exit interaction attempt timed out in UI automation, so those are not claimed as visually verified. Windows and Omarchy remain untested for this experiment. Dependency `block 0.1.6` produces a Rust future-incompatibility warning, though the current build succeeds.

## Linux Wayland compatibility fallback

The pinned Wayland window-decoration chain (`winit 0.27.5` → `sctk-adwaita` → `crossfont` → `freetype-rs 0.26`) aborts on current Omarchy while drawing title text, before gameplay. This is not a pong-core bug. The executable now chooses **process-local X11/XWayland** when both `WAYLAND_DISPLAY` and `DISPLAY` exist and `WINIT_UNIX_BACKEND` is unset. It logs that choice before initializing graphics. It does not change Hyprland, shell profiles, or global environment settings.

An explicit `WINIT_UNIX_BACKEND` always wins. To select X11 yourself:

```sh
WINIT_UNIX_BACKEND=x11 cargo run --manifest-path experiments/bracket-lib/Cargo.toml
```

Native Wayland is still not fixed; forcing `WINIT_UNIX_BACKEND=wayland` may reproduce the dependency abort. A Wayland-only session without XWayland cannot use this workaround. We do not disable Rust safety checks or retry after a non-unwinding abort.

Omarchy ARM64 verification: 5 tests passed, debug build passed; normal launch with no backend override logged the automatic fallback, initialized OpenGL 4.6 Mesa and survived the intentional six-second timeout (exit124). Parent environment remained unchanged. This verifies startup only, **not visual/gameplay QA**. Graphical startup requires access to the session display; isolated commands without that access fail with `XOpenDisplayFailed`.

## Native Wayland compatibility backport (2026-11-11)

The pinned windowing stack uses `freetype-rs 0.26.0`, whose empty-glyph buffer handling can abort under current Rust safety checks. This standalone manifest patches that crate to the MIT-attributed copy in `../../vendor/freetype-rs`; see its `PONG-PATCH.md`. The narrow change matches empty-buffer behavior in upstream 0.38.0 without forcing an incompatible dependency upgrade. Three dependency regression tests cover empty and populated glyphs.

On Omarchy ARM64, the original empty-glyph test reproduced the abort; the backport passes and explicit `WINIT_UNIX_BACKEND=wayland` initializes OpenGL and survives an eight-second native smoke test. This is not visual gameplay verification. The existing automatic X11 fallback and explicit-backend override remain unchanged. To exercise native Wayland, set `WINIT_UNIX_BACKEND=wayland` for this process only. macOS and Windows do not use this crossfont FreeType dependency.

## Windows verification (2026-11-11)

From checkpoint 4707ee9, locked MSVC full build passed and all five frontend tests passed. Supported Computer Use showed the court, paddles and ball; Space changed Ready to Playing, scoring advanced, Space paused, and R reset scores to 0:0. Escape closed the process/window (absence checked afterward). However, header/control text is visibly overlapping/garbled on this Windows display, so visual quality is NOT a clean pass. Sustained paddle movement/release, focus, resize and complete-match interaction remain unverified. No unsupported input or system changes were used.

### Text-rendering investigation

Tested the bundled 8x8 font at explicit 16x16 and 12x12 display-cell sizes through supported Windows UI. Integer 2x scaling made the text more legible, but the 1440x880 client exceeded this desktop height; 1.5x scaling fit but remained visually uneven. These experiments did not establish a satisfactory fix or a driver-level cause, so their source changes were reverted rather than shipped. The original build was restored. A properly designed readable header/font layout remains follow-up work; no global display/driver changes were attempted.

### Separate header console follow-up

The header now uses a separate transparent 8x16 VGA-font console over the original 8x8 court grid. Four short header lines have dedicated double-height rows and a blank separation before the court; the client remains a manageable 720x480 rather than enlarging the whole game to 1440x880. Explicit foreground colors avoid inheriting cell state. A layout regression checks ASCII, line widths and header/court separation across all game states. Windows locked MSVC tests pass 6/6 and full build passes; native screenshots confirmed separated, more legible text, start, score advancement, pause and reset. The bitmap glyphs remain visibly rough at this size, so this is a readability/layout improvement, not a claim that font quality or driver rasterization is fully resolved. Escape closed the app. Mac/Omarchy need rechecking for this presentation change; held input/full-match/focus/resize acceptance remains outstanding.
