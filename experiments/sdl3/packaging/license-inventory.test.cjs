const {test}=require('node:test');
const assert=require('node:assert/strict');
const {inventory}=require('./license-inventory.cjs');
test('inventory filters resolved packages and never exports private paths or package IDs',()=>{
  const result=inventory({resolve:{nodes:[{id:'path+file:///private/person/project#app'}]},packages:[
    {id:'path+file:///private/person/project#app',name:'app',version:'1',license:null,manifest_path:'/private/person/project/Cargo.toml',license_file:'/private/secret'},
    {id:'unused',name:'unused',version:'1',license:'MIT'}]});
  assert.equal(result.packages.length,1);assert.deepEqual(result.missing_license_declarations,['app@1']);
  assert.equal(result.packages[0].has_license_file,true);assert.equal(JSON.stringify(result).includes('/private'),false);
});
test('inventory preserves compound declarations and deterministic ordering',()=>{
  const m={resolve:{nodes:[{id:'b'},{id:'a'}]},packages:[{id:'b',name:'z',version:'2',source:'registry+https://example',license:'MIT OR Apache-2.0'},{id:'a',name:'a',version:'1',source:'git+https://example',license:'Zlib'}]};
  const r=inventory(m);assert.deepEqual(r.packages.map(p=>p.name),['a','z']);assert.equal(r.packages[1].license,'MIT OR Apache-2.0');assert.deepEqual(r.missing_license_declarations,[]);
});
test('unresolved metadata is refused',()=>assert.throws(()=>inventory({packages:[]}),/resolved/));
