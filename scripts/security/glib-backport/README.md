# GLib backport regression

Run from the repository root on a machine with Rust and GLib, GObject and
GIO development files available through pkg-config:

```sh
cargo test --locked --release --manifest-path scripts/security/glib-backport/Cargo.toml
```

Five focused tests exercise string iteration in both directions, mixed ends,
Unicode/empty strings, singleton/empty arrays, exhaustion, last/nth/nth_back,
and overflowing skips. Release optimization is essential: the upstream
immutable out-pointer bug can be masked by debug builds.

This isolated harness uses only the patched GLib package and does not
compile the Dioxus GUI. Passing it is not a full Dioxus integration or
binary-release audit. Preserve the upstream licenses and PATCH.md.

The additional upstream_variant_iter integration test preserves all 11 upstream
variant_iter tests, with only the crate-to-glib import adjusted. This avoids
requiring unrelated upstream benchmark/compiletest development dependencies.
