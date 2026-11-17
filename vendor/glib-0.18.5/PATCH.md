# GLib 0.18.5 VariantStrIter backport

This is the crates.io `glib 0.18.5` source archive, SHA-256
`233daaf6e83ae6a12a52055f568f9d7cf4671dabb78ff9560ab6da230ce00ee5`,
upstream revision `42b9caf98e03ded086362d9653ca58fe94dc8658`.
All upstream files, including COPYRIGHT and LICENSE, are retained.

The only upstream code changes are the two lines from gtk-rs/gtk-rs-core
commit `b5a4071e439bef2b5eea76c3aa25e5ae84839e34` (PR 1343): make the output
pointer mutable and pass `&mut p` to `g_variant_get_child` in
`VariantStrIter::impl_get`. This addresses GHSA-wrw7-89jp-8q8g /
RUSTSEC-2024-0429 without changing the GTK3-compatible 0.18 API.

Only the Dioxus experiment selects this patch. The focused release-mode
regression harness is in `scripts/security/glib-backport`.
This is not a general GTK, Dioxus, dependency, or distribution security
clearance. Version-based scanners may continue reporting 0.18.5; the
version is intentionally not falsified to hide the advisory.
