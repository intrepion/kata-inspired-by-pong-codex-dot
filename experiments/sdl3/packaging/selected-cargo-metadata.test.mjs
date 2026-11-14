import test from 'node:test';
import assert from 'node:assert/strict';
import {selectCargoMetadata} from './selected-cargo-metadata.mjs';
const p=(name)=>({id:name,name,version:'1.0.0'});
const node=(id,deps=[])=>({id,deps:deps.map(pkg=>({pkg})),dependencies:deps});
const metadata={packages:['game','sdl3','sdl3-image','sdl3-mixer','sdl3-ttf'].map(p),
 resolve:{root:'game',nodes:[node('game',['sdl3']),node('sdl3',['sdl3-image','sdl3-mixer','sdl3-ttf']),...['sdl3-image','sdl3-mixer','sdl3-ttf'].map(x=>node(x))]}};
test('optional packages are omitted only when absent from selected tree',()=>{
 for (const selected of [[],['sdl3-image'],['sdl3-mixer','sdl3-ttf'],['sdl3-image','sdl3-mixer','sdl3-ttf']]) {
  const tree=['game','sdl3',...selected].map(n=>n+' v1.0.0').join('\n');
  const actual=selectCargoMetadata(metadata,tree);
  assert.deepEqual(actual.packages.map(p=>p.name),['game','sdl3',...selected].sort((a,b)=>metadata.packages.findIndex(p=>p.name===a)-metadata.packages.findIndex(p=>p.name===b)));
  assert.deepEqual(actual.resolve.nodes.find(n=>n.id==='sdl3').dependencies,selected);
 }
});
test('unknown, ambiguous and malformed tree entries fail closed',()=>{
 assert.throws(()=>selectCargoMetadata(metadata,'game v1.0.0\nunknown v1.0.0'),/Missing/);
 assert.throws(()=>selectCargoMetadata({...metadata,packages:[...metadata.packages,p('sdl3')]},'game v1.0.0\nsdl3 v1.0.0'),/ambiguous/);
 assert.throws(()=>selectCargoMetadata(metadata,'not a valid tree'),/Unrecognized/);
});
