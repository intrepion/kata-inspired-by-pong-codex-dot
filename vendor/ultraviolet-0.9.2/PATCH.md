# Pong scoped ultraviolet 0.9.2 patch

Published crate SHA-256: 6a28554d13eb5daba527cc1b91b6c341372a0ae45ed277ffb2c6fbc04f319d7e.
The published manifest declares MIT OR Apache-2.0 OR Zlib. This integration selects the declared Apache-2.0 alternative and includes its canonical terms as LICENSE-APACHE-2.0; that file is not represented as a project-authored notice. The published package did not contain a project license text. Existing authorship, source comments, manifest and .cargo_vcs_info.json are retained. This is not a claim of complete attribution clearance.

Compared with the published crate, src/mat.rs removes scalar/SIMD into_rotor3 and dependent extract_rotation/into_isometry APIs containing or invoking the cited EuclideanSpace adaptation. The isometry test instead checks retained matrix translation and point transformation. src/util.rs has the reviewed narrow dead-code allowance(s). These are breaking public API removals, scoped to the Pong consumer via its [patch.crates-io] entry; no general downstream compatibility is claimed. Registry cache marker .cargo-ok is omitted.

Windows consumer tests and scripted replay were reported successful. Ten default-feature library tests passed in a separate fixture omitting the unavailable serde_test dev dependency; Pixels also changed fixture-only cached dependencies. Those fixture files and logs are NOT included here. Exact original-manifest, Serde and all-features tests are not qualified. Source integration does not qualify earlier release binaries or change their provenance.
