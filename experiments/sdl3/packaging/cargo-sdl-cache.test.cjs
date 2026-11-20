const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const findCache = require('./cargo-sdl-cache.cjs');
const id = 'path+file:///example/vendor/sdl3-sys#0.1.0';
const event = (out_dir, package_id = id) => JSON.stringify({reason:'build-script-executed',package_id,out_dir});
function fixture(t, ...parts) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(),'pong-cmake-cache-test-'));
  t.after(() => fs.rmSync(root,{recursive:true,force:true}));
  const release = path.join(root,'aarch64-apple-darwin','release');
  const out = path.join(release,'build',...parts,'out');
  const cache = path.join(out,'build','CMakeCache.txt');
  fs.mkdirSync(path.dirname(cache),{recursive:true});
  fs.writeFileSync(cache,'SDL_HIDAPI_LIBUSB:BOOL=OFF\n');
  return {root,release,out,cache};
}
for (const parts of [['sdl3-sys-oldhash'], ['sdl3-sys','newhash']]) {
  test('resolves Cargo event with '+parts.join('/'), t => {
    const {release,out} = fixture(t,...parts);
    assert.equal(findCache(event('/other','other')+'\n'+event(path.join(path.dirname(path.dirname(release)),'release','build','host','out'))+'\n'+event(out),id,release),'SDL_HIDAPI_LIBUSB:BOOL=OFF\n');
  });
}
test('missing exact ID or duplicate events fail closed', t => {
  const {release,out} = fixture(t,'build-id');
  assert.throws(() => findCache(event(out,'wrong'),id,release),/exactly one/);
  assert.throws(() => findCache(event(out)+'\n'+event(out),id,release),/exactly one/);
});
test('rejects sibling prefix and relative out_dir', t => {
  const {release} = fixture(t,'build-id');
  assert.throws(() => findCache(event(release+'-evil/build/id/out'),id,release),/outside/);
  assert.throws(() => findCache(event('build/id/out'),id,release),/invalid/);
});
test('rejects symlink cache and intermediate directory', t => {
  const {cache,out,release,root} = fixture(t,'build-id');
  const external = path.join(root,'outside');
  fs.renameSync(cache, external);
  fs.symlinkSync(external, cache);
  assert.throws(() => findCache(event(out),id,release),/symlink/);
  fs.renameSync(out, path.join(root,'real-out'));
  fs.symlinkSync(path.join(root,'real-out'),out);
  assert.throws(() => findCache(event(out),id,release),/symlink/);
});
