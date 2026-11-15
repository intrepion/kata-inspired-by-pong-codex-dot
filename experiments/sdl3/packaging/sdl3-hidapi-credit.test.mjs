import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtempSync, mkdirSync, writeFileSync, rmSync, realpathSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join, dirname} from 'node:path';
import {createHash} from 'node:crypto';
import {appendSdlHidapiCredit} from './sdl3-hidapi-credit.mjs';
const hash = v => createHash('sha256').update(v).digest('hex');

test('preserves exact source credit and rejects tampering', t => {
  const root = realpathSync(mkdtempSync(join(tmpdir(), 'sdl-hidapi-credit-')));
  t.after(() => rmSync(root, {recursive:true, force:true}));
  const path = 'SDL/src/hidapi/mac/hid.c', file = join(root,path);
  const comment = '/* Credit used with permission. */', bytes = 'prefix\n'+comment+'\nsuffix\n';
  mkdirSync(dirname(file),{recursive:true});writeFileSync(file,bytes);
  const entry = [path,hash(bytes),hash(comment),7,7+comment.length];
  const bundle = {text:'original',index:{packages:[{name:'sdl3-src',version:'3.4.20',notices:[]}]}};
  appendSdlHidapiCredit(bundle,root,entry);
  assert(bundle.text.includes(comment));
  assert.equal(bundle.index.packages[0].notices[0].sha256,hash(comment));
  assert.match(bundle.index.packages[0].notices[0].link_status,/final linkage unverified/);
  assert.throws(()=>appendSdlHidapiCredit({text:'',index:{packages:[]}},root,entry),/Expected one/);
  writeFileSync(file,bytes+'changed');
  assert.throws(()=>appendSdlHidapiCredit(bundle,root,entry),/source hash mismatch/i);
  writeFileSync(file,bytes);
  assert.throws(()=>appendSdlHidapiCredit(bundle,root,[path,hash(bytes),hash('wrong'),7,7+comment.length]),/comment hash mismatch/i);
});
