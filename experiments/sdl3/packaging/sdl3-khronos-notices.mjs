// Copyright/permission comments from SDL 3.4.20 Khronos headers listed in
// this Mac build's static-object dependency files. Header use does not prove
// their copyrightable contents are present in the final executable.
import {readFileSync, lstatSync, realpathSync} from 'node:fs';
import {resolve} from 'node:path';
import {createHash} from 'node:crypto';
import {auditedSourceComments} from '../../../scripts/packaging/notice-bundle.mjs';
const hash = data => createHash('sha256').update(data).digest('hex');
const APACHE_HASH = 'cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30';
export const SDL_3_4_20_KHRONOS_NOTICES = Object.freeze([
  [
    "SDL/src/video/khronos/EGL/egl.h",
    "f5f41711f5d7517eb50d63726d786d471a3e84a52b94fd9aca9b6262b629347c",
    "e3b26c62a8b050f7e53c28a026ae58b7dedda912035a4170914a5bae5f58ddb9",
    77,
    489
  ],
  [
    "SDL/src/video/khronos/EGL/eglext.h",
    "efc117e5cbbf21fa4f2e9dbfb3899eedb3235c0152a2f6d9ca4510b7505e400f",
    "e3b26c62a8b050f7e53c28a026ae58b7dedda912035a4170914a5bae5f58ddb9",
    83,
    495
  ],
  [
    "SDL/src/video/khronos/EGL/eglplatform.h",
    "25b5391655effcf363a38fa00c3154f356a3eab3d35dc067847875513cf1e441",
    "4c161ffa310e57ca2023b8065e96c8f4617a5231b0e142c1187c3b0381a9b183",
    51,
    141
  ],
  [
    "SDL/src/video/khronos/GLES2/gl2.h",
    "3a2855122942ddc6bed5195cae901a8468e7c633ac999e3f3d70b98949a28652",
    "ec1b7a05b185504e74ac1d35bc90b2007aed4cce8908ebc1572047fe49cb5209",
    89,
    427
  ],
  [
    "SDL/src/video/khronos/GLES2/gl2ext.h",
    "574de987d1f0751a132406abd893928cc7bc5b15c1df8ecea959f2732631b184",
    "ec1b7a05b185504e74ac1d35bc90b2007aed4cce8908ebc1572047fe49cb5209",
    95,
    433
  ],
  [
    "SDL/src/video/khronos/GLES2/gl2platform.h",
    "f5da0747540a50be5f44aad264aae45bdf157a192c40f17487dd9a2f99c71b6c",
    "3fd4bc0fa79b9de66bf5c7fb9e2e1c8d06e147b69e9942f4ee655350b7d1bf50",
    51,
    141
  ],
  [
    "SDL/src/video/khronos/KHR/khrplatform.h",
    "7b1e01aaa7ad8f6fc34b5c7bdf79ebf5189bb09e2c4d2e79fc5d350623d11e83",
    "fd7c88f69e70ffa778137ba0bc8267fa8dd83189365d9ff5a2533161432b3d0b",
    51,
    1197
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codec_av1std.h",
    "f3ab53dbfdeb36a349f674656d50bd507f2e4b3c883343da74aace0df4f02a02",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    77,
    170
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codec_av1std_decode.h",
    "96f923dc01cd3e266c081d88fa020525a20d63ffe4ce09a4c807a04ef3aaf97e",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    91,
    184
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codec_h264std.h",
    "072ee6eac80e73d93bb4bfe0059f99e581fc8b55b1437236185bb4ff377914b1",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    79,
    172
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codec_h264std_decode.h",
    "e1708b3996a740eeedb79f51d0f0e379f9bcae13566538375acaf73301023b58",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    93,
    186
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codec_h264std_encode.h",
    "052a6e357be8d88bf4be0db1b3da5a9cc1a6011cf4b3533759ff588e1e26f9f1",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    93,
    186
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codec_h265std.h",
    "0cbdfd6c45ee8ca902e9c961cd7e49a4d601d443ea0ef2a60a8496b127ed397c",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    79,
    172
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codec_h265std_decode.h",
    "800946aa19d83d305d504e2e4141bc938f989e0cf8872cef97480238eefef1ea",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    93,
    186
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codec_h265std_encode.h",
    "7239f9f4e811546639111e562a8be1ec73ad5bef1c23d5fad80889f35e125e9c",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    93,
    186
  ],
  [
    "SDL/src/video/khronos/vk_video/vulkan_video_codecs_common.h",
    "78cf3c99d3d57afa7ab1782ec6018b4bcdbe273f2843d76e3375282db875b71e",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    79,
    172
  ],
  [
    "SDL/src/video/khronos/vulkan/vk_platform.h",
    "c4cabbcf699c90cde344095ae54da9435d117951aad114ba3a540c5997d7906f",
    "74c1ab4270c3639822baf5b4601d0e3cfc61bea351a0a7130c7bc493d65b66fd",
    29,
    122
  ],
  [
    "SDL/src/video/khronos/vulkan/vulkan.h",
    "52297513cc6b6cc8db2921aa849f72e3f55d678b3a58a2c3eb297c02ded7a837",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    39,
    132
  ],
  [
    "SDL/src/video/khronos/vulkan/vulkan_beta.h",
    "fd7eaa882fb01dd0b798f3d11aeba0f20fc7de1b27dff50cae0a2b8ceb3ed373",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    49,
    142
  ],
  [
    "SDL/src/video/khronos/vulkan/vulkan_core.h",
    "439a6b8cb5c9688404947c9173a47c8a2d81a6382b5ddb514dc322b2400d9528",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    49,
    142
  ],
  [
    "SDL/src/video/khronos/vulkan/vulkan_macos.h",
    "5a2ac88f5ac809dab80a6353240302fac1d2fd7010eb377560996a12d392b030",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    51,
    144
  ],
  [
    "SDL/src/video/khronos/vulkan/vulkan_metal.h",
    "ba1b8ef95c08ab6cf6213cb783d0bc54f37986980df94c13cee887decfc04a89",
    "68a3c97bb062374950828521ba0ce8efefbc160b2258efcbf493ce1ad6626d5f",
    51,
    144
  ]
]);
export function collectSdlKhronosNotices(root, entries = SDL_3_4_20_KHRONOS_NOTICES) {
  return auditedSourceComments(root, entries).map((f,i) => ({
    ...f, source_sha256: entries[i][1],
    link_status: 'header in compiled-object dependencies; final linkage unverified'
  }));
}
export function appendSdlKhronosNotices(bundle, sourceRoot, apacheFile, entries = SDL_3_4_20_KHRONOS_NOTICES) {
  const pkgs = bundle.index.packages.filter(p => p.name === 'sdl3-src' && p.version === '3.4.20');
  if (pkgs.length !== 1) throw Error('Expected one SDL 3.4.20 source package');
  const notices = collectSdlKhronosNotices(sourceRoot, entries);
  const file = resolve(apacheFile);
  if (lstatSync(file).isSymbolicLink() || realpathSync(file) !== file) throw Error('Symlink Apache terms refused');
  const bytes = readFileSync(file);
  if (hash(bytes) !== APACHE_HASH) throw Error('Apache terms hash mismatch');
  const text = bytes.toString('utf8');
  if (!Buffer.from(text,'utf8').equals(bytes)) throw Error('Invalid Apache terms');
  for (const n of notices) {
    bundle.text += '\n===== sdl3-src 3.4.20 | SUPPLEMENTAL KHRONOS HEADER NOTICE | '+n.source+' =====\n'+n.text+'\n';
    const {text: _text, ...record} = n;
    pkgs[0].notices.push(record);
  }
  bundle.text += '\n===== Apache-2.0 full terms for identified Khronos headers =====\n'+text+'\n';
  pkgs[0].notices.push({source:'audited Apache-2.0 terms',sha256:APACHE_HASH,bytes:bytes.length,supplemental:true,source_notice:true,link_status:'header in compiled-object dependencies; final linkage unverified'});
}
