import {auditedSourceComments} from '../../../scripts/packaging/notice-bundle.mjs';

// SDL 3.4.20's macOS HID source is listed in compiled-object dependencies.
// This preserves its attribution; it does not assert a separate obligation
// or establish retention of this particular implementation in the binary.
export const SDL_HIDAPI_CREDIT = Object.freeze([
  'SDL/src/hidapi/mac/hid.c',
  '8f1ec426a744885896e0d8a37f75538a08307d67070f40088e8fa4009220d9f9',
  '6bb6588199169d7225e6fb5bdaab035b2efde68c93d54d97eb36b1d9793cb309',
  1172, 1430
]);

export function appendSdlHidapiCredit(bundle, sourceRoot, entry = SDL_HIDAPI_CREDIT) {
  const packages = bundle.index.packages.filter(p => p.name === 'sdl3-src' && p.version === '3.4.20');
  if (packages.length !== 1) throw Error('Expected one SDL 3.4.20 source package');
  const [credit] = auditedSourceComments(sourceRoot, [entry]);
  bundle.text += '\n===== sdl3-src 3.4.20 | SUPPLEMENTAL HIDAPI SOURCE ATTRIBUTION | '+credit.source+' =====\n'+credit.text+'\n';
  const {text: _text, ...record} = credit;
  packages[0].notices.push({...record, source_sha256:entry[1], link_status:'compiled-object dependency; final linkage unverified'});
}
