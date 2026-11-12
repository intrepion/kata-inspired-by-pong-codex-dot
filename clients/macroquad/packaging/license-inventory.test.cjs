const {test}=require('node:test'), assert=require('node:assert/strict');
const {scopedInventory}=require('./license-inventory.cjs');
const fixture=()=>({packages:[{id:'a',name:'pong-macroquad',version:'1',license:'MIT'},{id:'b',name:'dep',version:'2',license:null,manifest_path:'/private/secret'},{id:'c',name:'unrelated',version:'1'}],resolve:{nodes:[{id:'a',dependencies:['b']},{id:'b',dependencies:['a']},{id:'c',dependencies:[]}]}});
test('closure excludes unrelated workspace apps and handles cycles',()=>{const r=scopedInventory(fixture());assert.equal(r.packages.length,2);assert.deepEqual(r.missing_license_declarations,['dep@2']);assert(!JSON.stringify(r).includes('/private'));});
test('missing root fails closed',()=>{const m=fixture();m.packages.shift();assert.throws(()=>scopedInventory(m));});
test('missing dependency node fails closed',()=>{const m=fixture();m.resolve.nodes.splice(1,1);assert.throws(()=>scopedInventory(m));});
