const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {tmpdir} = require('node:os');
const {spawnSync} = require('node:child_process');

const destinations = [
  'experiments/native-swift/App/Resources/smoke-v1.txt',
  'experiments/native-swift/Tests/PongCoreTests/Fixtures/smoke-v1.txt',
  'experiments/native-kotlin/app/src/test/resources/smoke-v1.txt',
  'experiments/native-kotlin/app/src/main/assets/scenarios/smoke-v1.txt',
];
function fixture(t, withSource = true) {
  const root = fs.mkdtempSync(path.join(tmpdir(), 'pong-mobile-scenario-'));
  t.after(() => fs.rmSync(root, {recursive:true, force:true}));
  fs.mkdirSync(path.join(root, 'scripts'), {recursive: true});
  fs.copyFileSync(path.join(__dirname, 'package-mobile-scenario.cjs'), path.join(root, 'scripts', 'package-mobile-scenario.cjs'));
  if (withSource) {
    fs.mkdirSync(path.join(root, 'scenarios'));
    fs.writeFileSync(path.join(root, 'scenarios', 'smoke-v1.txt'), 'fixture scenario\n');
  }
  return root;
}
function run(root, ...args) {
  return spawnSync(process.execPath, [path.join(root, 'scripts', 'package-mobile-scenario.cjs'), ...args], {encoding: 'utf8'});
}
function installSentinels(root) {
  for (const dest of destinations) {
    const p = path.join(root, dest);
    fs.mkdirSync(path.dirname(p), {recursive: true});
    fs.writeFileSync(p, `untouched:${dest}\n`);
  }
}
test('valid generation and check work against isolated fixture', t => {
  const root = fixture(t);
  const generated = run(root);
  assert.equal(generated.status, 0, generated.stderr);
  assert.match(generated.stdout, /generated: 4/);
  for (const dest of destinations) assert.equal(fs.readFileSync(path.join(root, dest), 'utf8'), 'fixture scenario\n');
  const checked = run(root, '--check');
  assert.equal(checked.status, 0, checked.stderr);
  assert.match(checked.stdout, /verified: 4/);
});
test('unknown, duplicated and combined arguments fail before overwriting files', t => {
  const root = fixture(t); installSentinels(root);
  for (const args of [['--chek'], ['--dry-run'], ['--check','--check'], ['--check','--unknown'], ['extra']]) {
    const result = run(root, ...args);
    assert.notEqual(result.status, 0, args.join(' '));
    assert.match(result.stderr, /Usage:/);
    for (const dest of destinations) assert.equal(fs.readFileSync(path.join(root, dest), 'utf8'), `untouched:${dest}\n`);
  }
});
test('invalid arguments are rejected before reading even a missing source', t => {
  const root = fixture(t, false);
  const result = run(root, '--chek');
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Usage:/);
  assert.doesNotMatch(result.stderr, /ENOENT/);
  for (const dest of destinations) assert.equal(fs.existsSync(path.join(root, dest)), false);
});
test('check rejects stale or missing resources without rewriting them', t => {
  const root = fixture(t);
  assert.notEqual(run(root, '--check').status, 0);
  assert.equal(run(root).status, 0);
  const stale = path.join(root, destinations[1]);
  fs.writeFileSync(stale, 'stale fixture\n');
  const result = run(root, '--check');
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /Stale scenario resource/);
  assert.equal(fs.readFileSync(stale, 'utf8'), 'stale fixture\n');
});
