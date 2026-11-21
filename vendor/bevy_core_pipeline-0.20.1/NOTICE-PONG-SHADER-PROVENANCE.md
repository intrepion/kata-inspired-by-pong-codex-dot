# Third-party material retained in Pong's scoped Bevy core pipeline

The adjacent files preserve the complete original notice blocks or license file
bytes. This documentation adds attribution to existing Bevy adaptations; it
does not add a new copy of the original third-party implementation and does
not change the inherited shader bytes.

- `src/mip_generation/downsample.wesl` is Bevy's WESL port of the AMD
  FidelityFX Single Pass Downsampler in
  [`ffx_spd.h` at `c16b1d286b5b438b75da159ab51ff426bacea3d1`](https://github.com/GPUOpen-LibrariesAndSDKs/FidelityFX-SDK/blob/c16b1d286b5b438b75da159ab51ff426bacea3d1/sdk/include/FidelityFX/gpu/spd/ffx_spd.h),
  Git blob `c3ee50f02734801e71bbb16efbefef396e53c06e`. The complete original
  2024 copyright and permission text is in
  [LICENSE-AMD-FidelityFX-SPD-2024.txt](LICENSE-AMD-FidelityFX-SPD-2024.txt).
- `src/mip_generation/experimental/downsample_depth.wesl` is Bevy's WESL
  depth-downsampling adaptation of AMD FidelityFX SPD v2.1, based on
  [`ffx_spd.h` at `d7531ae47d8b36a5d4025663e731a47a38be882f`](https://github.com/GPUOpen-LibrariesAndSDKs/FidelityFX-SDK/blob/d7531ae47d8b36a5d4025663e731a47a38be882f/sdk/include/FidelityFX/gpu/spd/ffx_spd.h),
  Git blob `0fc67c5d236498d646f9781b6b788cbc19219f87`. The complete original
  2023 copyright and permission text is in
  [LICENSE-AMD-FidelityFX-SPD-2023.txt](LICENSE-AMD-FidelityFX-SPD-2023.txt).
- `src/oit/resolve/oit_resolve.wesl` cites the Vulkan-Samples
  order-independent transparency linked-list blend shader. Bevy's WESL
  adaptation integrates with Bevy's sorting, blending, and rendering
  pipeline rather than using the original GLSL shader unchanged. Its source
  citation does not identify an exact donor revision. For conservative
  coverage, preserve the source notice from the
  [introduction revision `15b87879b2b2eeef74641c6d1185bc86c19d3694`](https://github.com/KhronosGroup/Vulkan-Samples/blob/15b87879b2b2eeef74641c6d1185bc86c19d3694/shaders/oit_linked_lists/combine.frag),
  blob `0989dd1d624038bcb492aafe734153ac7d72a3a0` (Google 2023), and the
  [later reviewed revision `0a97dba1e71b8fa191afb012b784ebad477ccf5a`](https://github.com/KhronosGroup/Vulkan-Samples/blob/0a97dba1e71b8fa191afb012b784ebad477ccf5a/shaders/oit_linked_lists/combine.frag),
  blob `cc6cd8bcfca98dbab0d92ff16d65a02c7a12c4ed` (Google 2023–2024).
  The original source headers and common complete Apache-2.0 license
  (blob `d3f888a0b2d042b2cd326bf2f45b644a62f19277` at both revisions) are
  included alongside this file. No separate upstream NOTICE was located
  at either reviewed revision.

These notices cover the listed files, not all of Bevy or the final linked
package. Ship the original notice files alongside the final artifact when
those files are retained; they are not runtime shader dependencies.
