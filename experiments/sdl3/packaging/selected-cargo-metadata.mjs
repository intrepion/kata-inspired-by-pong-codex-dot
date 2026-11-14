// The no-dedupe Cargo tree identifies the packages selected for the same
// target and dependency kinds used by the source build. Metadata alone may
// include optional packages that this build does not select.
export function selectCargoMetadata(metadata, tree) {
 if (!Array.isArray(metadata?.packages) || !Array.isArray(metadata.resolve?.nodes) || typeof tree !== 'string') throw Error('Invalid Cargo inputs');
 const keys = new Set();
 for (const line of tree.split(/\r?\n/)) {
  if (!line.trim()) continue;
  const match = /^([A-Za-z0-9_-]+) v(\S+)(?:\s|$)/.exec(line);
  if (!match) throw Error('Unrecognized Cargo tree line');
  keys.add(match[1]+'@'+match[2]);
 }
 if (!keys.size) throw Error('Empty Cargo tree');
 const counts = new Map();
 for (const p of metadata.packages) {
  const key = p.name+'@'+p.version;
  if (keys.has(key)) counts.set(key,(counts.get(key)||0)+1);
 }
 for (const key of keys) if (counts.get(key)!==1) throw Error('Missing or ambiguous selected Cargo package: '+key);
 const packages=metadata.packages.filter(p=>keys.has(p.name+'@'+p.version));
 const selected=new Set(packages.map(p=>p.id));
 const nodes=metadata.resolve.nodes.filter(n=>selected.has(n.id)).map(n=>({...n,
  deps:n.deps.filter(d=>selected.has(d.pkg)),dependencies:n.dependencies.filter(id=>selected.has(id))}));
 if(nodes.length!==selected.size || !selected.has(metadata.resolve.root)) throw Error('Selected graph is incomplete');
 return {...metadata,packages,resolve:{...metadata.resolve,nodes}};
}
