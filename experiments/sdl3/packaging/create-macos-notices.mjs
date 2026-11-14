import {readFileSync} from 'node:fs';
import {dirname, resolve} from 'node:path';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {auditedSourceComments, buildNoticeBundle} from '../../../scripts/packaging/notice-bundle.mjs';
import {selectCargoMetadata} from './selected-cargo-metadata.mjs';

const root = resolve(fileURLToPath(new URL('../../..', import.meta.url)));
const input = JSON.parse(readFileSync(0, 'utf8'));
const metadata = selectCargoMetadata(input.metadata, input.tree);
const result = buildNoticeBundle(metadata, root, {platform: 'macOS'});
result.index.scope = 'Cargo tree selected for aarch64-apple-darwin, normal and build dependencies; not linked-binary attribution';
// SDL's stb_image.h has its license at the end of the source, rather than in
// a separately named notice. Bind extraction to the exact audited source.
const source = metadata.packages.filter(p => p.name === 'sdl3-src' && p.version === '3.4.20' && p.source === 'registry+https://github.com/rust-lang/crates.io-index');
if (source.length !== 1) throw Error('Expected the audited SDL source package');
const [stb] = auditedSourceComments(dirname(source[0].manifest_path), [[
  'SDL/src/video/stb_image.h', '6741ecbd17740f45aa2fac0c9dc82d3079fe28860630a2755fe001827cc06ba5',
  '10ed5a55912965c9846c947486fc42a537c6e7f92dbabd68bf26963da320e46d', 289696, 292369
]]);
result.text += '\n===== sdl3-src 3.4.20 | SUPPLEMENTAL AUDITED SOURCE NOTICE | '+stb.source+' =====\n'+stb.text+'\n';
const {text: _text, ...indexed} = stb;
result.index.packages.find(p => p.name === 'sdl3-src' && p.version === '3.4.20').notices.push(indexed);
result.index.bundle_sha256 = createHash('sha256').update(result.text).digest('hex');
result.index.unresolved_findings = [
  'Cargo metadata is not linked-binary attribution; independently review SDL native source and bundled component notices.',
  'HIDAPI offers BSD-style, original, or GPLv3 terms; the BSD-style option is intended for this pilot and physical controller compatibility is unverified.',
  'Downloaded, signed, notarized, clean-machine, and other-platform packages are not qualified.',
  'Preinstalled macOS frameworks and system libraries are runtime requirements and are not bundled here.'
];
process.stdout.write(JSON.stringify(result));
