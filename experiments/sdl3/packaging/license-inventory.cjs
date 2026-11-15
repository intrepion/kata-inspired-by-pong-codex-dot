// Metadata inventory only, NOT license clearance or linked-binary attribution.
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
function inventory(metadata) {
  if (!Array.isArray(metadata.packages) || !metadata.resolve?.nodes) throw Error('resolved Cargo metadata required');
  const selected = new Set(metadata.resolve.nodes.map(n => n.id));
  const packages = metadata.packages.filter(p => selected.has(p.id)).map(p => ({
    name: p.name, version: p.version, license: p.license ?? null,
    origin: p.source?.startsWith('registry+') ? 'registry' : p.source?.startsWith('git+') ? 'git' : 'local',
    has_license_file: Boolean(p.license_file),
  })).sort((a,b) => a.name.localeCompare(b.name) || a.version.localeCompare(b.version));
  return {schema:1, scope:'Cargo resolved metadata graph; may include build/optional dependencies; not a linked-binary inventory',
    limitations:['SPDX declarations are not verified license texts','Bundled native sources and their third-party notices need separate inspection','Platform runtime libraries are not inventoried','No redistribution clearance implied'],
    packages, missing_license_declarations: packages.filter(p => !p.license).map(p => `${p.name}@${p.version}`)};
}
if (require.main === module) {
  const [output] = process.argv.slice(2);
  if (!output) throw Error('Usage: node license-inventory.cjs NEW_OUTPUT.json');
  const manifest = path.resolve(__dirname, '../Cargo.toml');
  const metadata = JSON.parse(cp.execFileSync('cargo', ['metadata','--offline','--locked','--format-version','1','--manifest-path',manifest,'--filter-platform','aarch64-unknown-linux-gnu'], {encoding:'utf8',maxBuffer:16*1024*1024}));
  fs.writeFileSync(output, JSON.stringify(inventory(metadata),null,2)+'\n', {flag:'wx'});
}
module.exports = {inventory};
