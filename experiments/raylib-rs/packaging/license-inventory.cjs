// Follow only raylib-rs's resolved dependency closure, not unrelated workspace apps.
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const {inventory} = require('../../sdl3/packaging/license-inventory.cjs');
function scopedInventory(metadata) {
  const root = metadata.packages.find(p => p.name === 'pong-raylib-rs');
  if (!root) throw Error('pong-raylib-rs package absent');
  const nodes = new Map(metadata.resolve.nodes.map(n=>[n.id,n]));
  const seen = new Set(), pending=[root.id];
  while (pending.length) { const id=pending.pop(); if(seen.has(id))continue; seen.add(id); const n=nodes.get(id); if(!n)throw Error('missing resolved node'); pending.push(...n.dependencies); }
  const result=inventory({...metadata,resolve:{nodes:metadata.resolve.nodes.filter(n=>seen.has(n.id))}});
  result.scope='raylib-rs resolved dependency closure for aarch64-apple-darwin; includes build dependencies, not a linked-binary inventory';
  return result;
}
if(require.main===module){
  const [out]=process.argv.slice(2);if(!out)throw Error('Usage: node license-inventory.cjs NEW_OUTPUT.json');
  const m=JSON.parse(cp.execFileSync('cargo',['metadata','--offline','--locked','--format-version','1','--manifest-path',path.resolve(__dirname,'../Cargo.toml'),'--filter-platform','aarch64-apple-darwin'],{encoding:'utf8',maxBuffer:32*1024*1024}));
  fs.writeFileSync(out,JSON.stringify(scopedInventory(m),null,2)+'\n',{flag:'wx'});
}
module.exports={scopedInventory};
