# Pong-scoped objc 0.2.7
The original crates.io archive SHA-256 is
`915b1b472bc21c53464d6c8461c9d3af805ba1ef837e1cac254428f4a77177b1`,
recorded revision `858f92f7c11deb4cd95fe8c8b58ff8e1a6758f5e`. The
original Steven Sheldon MIT LICENSE.txt is retained unchanged. The registry
manifest's malloc_buf dependency is removed; Cargo.toml.orig is archival and
the crate's standalone Cargo.lock is omitted (Pong owns the integration lock).

This one experiment reuses Pong's independently authored C allocation owner
from vendor/objc-rs-0.2.8, not malloc_buf code. It takes ownership of original
Objective-C runtime allocations and calls C free once. The six list methods
return COwnedSlice with read-only slice access; no mutable or raw ownership
API, nor Send or Sync assertion, is added. Callers must respect the runtime
allocation size, validity and lifetime guarantees. Method encodings preserve
their old contents and take ownership before checking UTF-8. This is a scoped
Pong fork, not a general-purpose drop-in objc update or a claim about objc2.
