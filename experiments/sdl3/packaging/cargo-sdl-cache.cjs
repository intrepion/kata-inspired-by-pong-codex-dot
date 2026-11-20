const fs = require('node:fs');
const path = require('node:path');

module.exports = function sdlCacheFromCargo(messages, packageId, release) {
  const matches = messages.split(/\r?\n/).filter(Boolean).map(line => JSON.parse(line))
    .filter(row => row.reason === 'build-script-executed' && row.package_id === packageId);
  const root = fs.realpathSync(release);
  const lexicalRoot = path.resolve(release);
  // Cargo can compile the same package for host build dependencies too.
  // Only the exact target event provides the library we will package.
  const hostBuild = path.join(path.dirname(path.dirname(lexicalRoot)), 'release', 'build');
  const forTarget = matches.filter(row => {
    if (typeof row.out_dir !== 'string' || !path.isAbsolute(row.out_dir)) return true;
    const relative = path.relative(hostBuild,row.out_dir);
    return relative === '' || path.isAbsolute(relative) || relative === '..' ||
      relative.startsWith('..'+path.sep);
  });
  if (forTarget.length !== 1) throw Error('expected exactly one target Cargo build-script event for vendored sdl3-sys');
  const out = forTarget[0].out_dir;
  if (typeof out !== 'string' || !path.isAbsolute(out)) throw Error('invalid SDL Cargo out_dir');
  const relative = path.relative(path.join(lexicalRoot,'build'), out);
  const parts = relative.split(path.sep);
  if (parts.length < 2 || path.isAbsolute(relative) || parts.includes('..') ||
      parts[parts.length-1] !== 'out') throw Error('SDL Cargo out_dir is outside the expected release build tree');
  let candidate = root;
  const components = ['build', ...parts, 'build', 'CMakeCache.txt'];
  for (const [index, part] of components.entries()) {
    candidate = path.join(candidate,part);
    const stat = fs.lstatSync(candidate);
    if (stat.isSymbolicLink() || !(index === components.length-1 ? stat.isFile() : stat.isDirectory()))
      throw Error('SDL CMake cache path contains a symlink or unexpected file type');
  }
  if (fs.realpathSync(candidate) !== candidate) throw Error('SDL CMake cache real path mismatch');
  return fs.readFileSync(candidate,'utf8');
};
