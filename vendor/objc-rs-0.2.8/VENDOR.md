# Vendored objc-rs 0.2.8

Source: https://static.crates.io/crates/objc-rs/objc-rs-0.2.8.crate
SHA-256: `64a1e7069a2525126bf12a9f1f7916835fafade384fb27cabf698e745e2a1eb8`.
The archive records upstream Git revision
[`9089e96c4fa88330d26d2fb4e4e826770a6f017f`](https://github.com/raphamorim/objc-rs/tree/9089e96c4fa88330d26d2fb4e4e826770a6f017f).
`LICENSE.txt` is the original MIT license, copied byte-for-byte.

The manifests remove the `malloc_buf` dependency and use a local,
C-allocated slice owner for existing Objective-C methods. The only
locally modified library files are `src/lib.rs`, `src/runtime.rs` and
`src/encode.rs`, plus new `src/c_owned_slice.rs`. Cargo's packaging
metadata, standalone lockfile, changelog and original duplicate manifest are omitted.

The Objective-C runtime supplies initialized arrays of borrowed runtime
references and malloc-owned NUL-terminated method encodings. The wrapper
does not transfer ownership of their referenced objects. It owns only
the original array/string allocation, and uses libc `free` once at drop,
including on a validation unwind. Empty slices use an aligned dangling
pointer without changing the value passed to `free`. Null nonempty,
misaligned, overflowing and out-of-range lengths are rejected before
creating a slice. FFI callers must still guarantee the actual allocation
extent, alignment, initialization, element validity and NUL termination.
The UTF-8 check runs after ownership is established, so a malformed
encoding does not leak.
