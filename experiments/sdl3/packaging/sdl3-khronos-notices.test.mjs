import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync, mkdirSync, realpathSync, rmSync, writeFileSync, copyFileSync, symlinkSync} from 'node:fs';
import {join, resolve} from 'node:path';
import {tmpdir} from 'node:os';
import {createHash} from 'node:crypto';
import {appendSdlKhronosNotices, collectSdlKhronosNotices, SDL_3_4_20_KHRONOS_NOTICES} from './sdl3-khronos-notices.mjs';

const hash = value => createHash('sha256').update(value).digest('hex');
test('manifest names each reviewed header once', () => {
  assert.equal(SDL_3_4_20_KHRONOS_NOTICES.length, 22);
  assert.equal(new Set(SDL_3_4_20_KHRONOS_NOTICES.map(x => x[0])).size, 22);
  for (const path of ['GLES2/gl2.h', 'GLES2/gl2ext.h', 'GLES2/gl2platform.h', 'KHR/khrplatform.h', 'vulkan/vulkan_core.h'])
    assert(SDL_3_4_20_KHRONOS_NOTICES.some(x => x[0].endsWith(path)));
});
test('exact source and copyright bytes are preserved and tampering fails closed', t => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), 'sdl-khronos-notice-')));
  t.after(() => rmSync(root, {recursive:true, force:true}));
  mkdirSync(join(root, 'src'));
  const notice = '/* Copyright Khronos. Permission notice. */';
  const content = 'prefix\n'+notice+'\ncode\n';
  const file = join(root, 'src', 'header.h');
  writeFileSync(file, content);
  const entry = ['src/header.h', hash(content), hash(notice), 7, 7+notice.length];
  const [found] = collectSdlKhronosNotices(root, [entry]);
  assert.equal(found.text, notice);
  assert.equal(found.source_sha256, hash(content));
  assert.match(found.link_status, /final linkage unverified/);
  const apache = join(root, 'apache.txt');
  copyFileSync(resolve('scripts/packaging/audited-notices/gl-rs-LICENSE.txt'), apache);
  const bundle = {text:'original\n',index:{packages:[{name:'sdl3-src',version:'3.4.20',notices:[]}],missing_notice_files:['other@1']}};
  appendSdlKhronosNotices(bundle, root, apache, [entry]);
  assert(bundle.text.includes(notice));
  assert(bundle.text.includes('Apache License'));
  assert.equal(bundle.index.packages[0].notices.length, 2);
  assert.deepEqual(bundle.index.missing_notice_files, ['other@1']);
  assert.throws(() => appendSdlKhronosNotices({text:'',index:{packages:[]}}, root, apache, [entry]), /Expected one/);
  writeFileSync(file, content+'changed');
  assert.throws(() => collectSdlKhronosNotices(root, [entry]), /source hash mismatch/i);
  writeFileSync(file, content);
  assert.throws(() => collectSdlKhronosNotices(root, [[...entry.slice(0,2), hash('wrong'), ...entry.slice(3)]]), /comment hash mismatch/i);
  assert.throws(() => collectSdlKhronosNotices(root, [['../outside', ...entry.slice(1)]]), /escapes/i);
  const alias = join(root, 'apache-link.txt');
  symlinkSync(apache, alias);
  assert.throws(() => appendSdlKhronosNotices(bundle, root, alias, [entry]), /Symlink Apache terms refused/);
  writeFileSync(apache, 'altered');
  assert.throws(() => appendSdlKhronosNotices(bundle, root, apache, [entry]), /Apache terms hash mismatch/);
});
