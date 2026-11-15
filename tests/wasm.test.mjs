import {readFile} from 'node:fs/promises';
import test from 'node:test';
import assert from 'node:assert/strict';
const bytes=await readFile(new URL('../clients/web-wasm/pong_wasm.wasm',import.meta.url));
async function game(){return (await WebAssembly.instantiate(bytes,{})).instance.exports;}
test('WASM ABI starts, moves, pauses, resumes and resets',async()=>{const g=await game();g.init(42);assert.equal(g.value(6),0);g.start();assert.equal(g.value(6),1);for(let i=0;i<30;i++)g.step(1/120,-1);assert.ok(g.value(0)<250);g.pause();const y=g.value(0);g.step(0.03,1);assert.equal(g.value(0),y);g.toggle_pause();assert.equal(g.value(6),1);g.start();assert.equal(g.value(0),250);assert.equal(g.value(4),0);assert.equal(g.value(5),0);});
test('independent WASM instances replay deterministically',async()=>{const a=await game(),b=await game();for(const g of [a,b]){g.init(123);g.start();for(let i=0;i<3600;i++)g.step(1/120,Math.sin(i/120)>0?1:-1);}for(let i=0;i<8;i++)assert.equal(a.value(i),b.value(i));});
