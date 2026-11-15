# Dioxus dirs-sys candidate

Selected files derive from dirs-sys 0.5.0, registry checksum e01a3366d27ee9890022452ee61b2b63a67e6f13f58900b651ff5665f0bb1fab, upstream revision 8bcd4aa2c35990d57a2cff2953793525fc42709c. MIT and Apache-2.0 licenses are retained. This reuses the preserved ggez patch: remove option-ext dependency and extern declaration, replace its XDG comparison with user_dir == Some(key), and retain its regression. The upstream README has a whitespace-only change.

Only the copied Dioxus manifest selects this candidate. The minimal manually reconciled lock preserves the original Windows selections and passes Cargo --locked checks for macOS desktop and WASM web; Windows/Linux metadata and tests remain unverified due to unavailable offline crates. The standalone vendor Cargo.lock was generated only by testing and is not part of the proposed patch. This is not complete license or release clearance.
