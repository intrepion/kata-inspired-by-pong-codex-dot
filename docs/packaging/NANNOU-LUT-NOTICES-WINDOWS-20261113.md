# Windows Nannou embedded lookup tables

See the [provenance follow-up](NANNOU-LUT-PROVENANCE-WINDOWS-20261113.md) for the source history, unresolved license evidence and feature path.

The historical [Windows pilot](NANNOU-HELD-WINDOWS-RELEASE-PILOT.md) executable SHA-256 `4b590de541004ee31f7989be554c64a5685ec26c67abbdd1368b8485fbd24624` contains complete cached `bevy_core_pipeline@0.19.1` LUT byte sequences:

| File | Bytes | Executable offset | SHA-256 |
| --- | ---: | ---: | --- |
| `Blender_-11_12.ktx2` | 308,985 | 48,624,640 | `793bd0e2b75c1eff50f9836c967f98e4e20ea2ff5c6cf5b5d0b2e7412387d3c6` |
| `AgX-default_contrast.ktx2` | 17,922 | 48,933,629 | `02b6997abe327ae0fd9af9dbb1c1ef4c4e1eac81daf0fb9f5f2c35f2b2afcfcf` |
| `tony_mc_mapface.ktx2` | 354,320 | 48,951,551 | `0a680861d32a617095fad3ab5af9b7bdb80771d118b84a2483a12f9960e461fe` |

The three cached files match upstream [Bevy revision b56fc29](https://github.com/bevyengine/bevy/tree/b56fc29d3016e641754765244b5ba3f9cc504671/crates/bevy_core_pipeline/src/tonemapping/luts) Git blobs `43bca82e4e7ce9a27f5005f97c781630845589bc`, `8633be506cd8f250cddd68af24b90ac997d01448` and `be665043fa421e86b1bb35b3730975557f584017` respectively. The matching upstream `info.txt` blob `e3b6b8a17f4296ab6ceb7d2338a1e93965f561ea` documents conversions from pinned AgX and Tony assets and a Blender Filmic process. These descriptions are provenance leads; we have not reproduced the conversions or established all underlying terms. The cached source includes all three files through `include_bytes!`.

The pinned [Tony source](https://github.com/h3r2tic/tony-mc-mapface/tree/909e51c8a74251fd828770248476cb084081e08c) describes its contribution as MIT or Apache-2.0. Its exact MIT and Apache license blobs are `473a6ba2f57f8e884a1c3543fe11eb8fc2970b3f` and `cf20408fd0c564ff633bd29d0da74d9e8f117b2c`; their SHA-256 hashes are `b00743beb398b2dac3fbc451e780b9f4cb0e9bdc04a27ff448e919b3bf1dc32a` and `e7edb2188aa1d7e1690a35d726f673fbc6555b2d1c26635adcdffd750a2c9e9d`. Both are included for review without a license choice. The pinned [AgXc tree](https://github.com/MrLixm/AgXc/tree/898198e0490b0551ed81412a0c22e0b72fffb7cd) has no license-named file in its recursive listing; its README calls it a fork of AgX. The AgX and Blender-derived LUT terms and attribution remain unresolved; the latter's generation using Blender does not by itself establish a GPL obligation.

A separate unsigned local candidate adds the two exact Tony texts to the prior font-discrepancy candidate and explicitly marks the LUT and font questions open. Its 21,817,466-byte ZIP SHA-256 is `68e31e7a77774f62f0a59f3ef7b0d20389b5680132b9a3b0f546926fd0e4216b`, and manifest SHA-256 is `a5dc25ab5eab4641856f9bc677faf6c13197380fe6445d02830d75a46c2f1605`. All seven payload hashes passed after extraction to a separate path containing a space. The executable is byte-identical; no new runtime or physical input test was performed. Candidate files are in sibling directory `nannou-lut-notice-candidate-windows-20261113`; the bounded byte-search script is preserved in `nannou-font-discrepancy-candidate-windows-20261113`. Earlier packages remain untouched. Complete-byte searches for five other cached Bevy LUTs returned no match, which does not exclude transformed assets or other dependencies.

The asset terms, eight resolved-graph missing-file flags, font discrepancy, other notices, linked and system dependencies, license review, signing, accessibility and clean-machine qualification remain open. This local candidate is not a public release or legal clearance.
