// Verify recorded capture inputs against Git history; current files may legitimately differ.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../', import.meta.url));
const manifest = JSON.parse(await readFile(new URL('../arcade/manifest.json', import.meta.url), 'utf8'));
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
let checked = 0;
const changed = new Set();
for (const image of manifest.screenshots) {
  const p = image.provenance;
  if (p?.kind !== 'ci_gameplay_capture') continue;
  assert.match(p.captureCommit, /^[a-f0-9]{40}$/);
  assert.ok(p.inputHashes && Object.keys(p.inputHashes).length);
  for (const [name, expected] of Object.entries(p.inputHashes)) {
    assert.match(name, /^[a-zA-Z0-9._/-]+$/);
    assert.ok(!name.startsWith('/') && !name.split('/').includes('..'));
    assert.match(expected, /^[a-f0-9]{64}$/);
    const recorded = execFileSync('git', ['show', `${p.captureCommit}:${name}`], { cwd: root });
    assert.equal(sha256(recorded), expected, `${image.path}: ${name} does not match capture commit`);
    const current = await readFile(new URL(`../${name}`, import.meta.url));
    if (sha256(current) !== expected) changed.add(name);
  }
  checked++;
}
console.log(`Verified historical input hashes for ${checked} CI captures.`);
console.log(changed.size ? `Current checkout differs: ${[...changed].sort().join(', ')}` : 'Current checkout matches recorded inputs.');
