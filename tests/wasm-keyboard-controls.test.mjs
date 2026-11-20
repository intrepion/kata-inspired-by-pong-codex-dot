import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import vm from 'node:vm';
import {MovementInput} from '../clients/web-wasm/input.mjs';

// Execute the actual browser entry point with a stub WASM engine and DOM.
// This checks event routing, not a claim of physical-input/browser qualification.
async function harness(realBytes) {
 const elements=new Map(), listeners=new Map();
 class Element {
  constructor(id){this.id=id;this.listeners=new Map();}
  addEventListener(type,fn){const list=this.listeners.get(type)||[];list.push(fn);this.listeners.set(type,list);}
  closest(selector){return ['start','pause','up','down'].includes(this.id)&&selector.includes('button')?this:null;}
  getContext(){return new Proxy({}, {get:()=>()=>{}});}
  setPointerCapture(){}
  emit(type,event={}){event.target??=this;event.preventDefault=()=>event.defaultPrevented=true;
   this['on'+type]?.(event);for(const fn of this.listeners.get(type)||[])fn(event);
   if(type!=='blur')for(const fn of listeners.get(type)||[])fn(event);return event;}
 }
 for(const id of ['court','status','start','pause','up','down'])elements.set(id,new Element(id));
 let state=1, starts=0, pauses=0, tracked, frame;
 const axes=[];
 class TrackedInput extends MovementInput {constructor(){super();tracked=this;}}
 const g={init(){},step(_dt,axis){axes.push(axis);},value(){return state;},start(){starts++;state=1;},toggle_pause(){pauses++;state=state===2?1:2;},pause(){state=2;}};
 const real = realBytes && (await WebAssembly.instantiate(realBytes, {})).instance.exports;
 const engine = real ? {...real,step(dt,axis){axes.push(axis);real.step(dt,axis);}} : g;
 const document={querySelector:s=>elements.get(s.slice(1)),getElementById:id=>elements.get(id),addEventListener(){}};
 const context=vm.createContext({MovementInput:TrackedInput,document,window:{},fetch:async()=>({ok:true,arrayBuffer:async()=>new ArrayBuffer(0)}),WebAssembly:{instantiate:async()=>({instance:{exports:engine}})},crypto:{getRandomValues:a=>a},URLSearchParams,location:{search:''},Uint32Array,requestAnimationFrame(fn){frame=fn;},console,addEventListener:(type,fn)=>{const list=listeners.get(type)||[];list.push(fn);listeners.set(type,list);}});
 const source=(await readFile(new URL('../clients/web-wasm/app.js',import.meta.url),'utf8')).replace(/^import .*\n/,'');
 await vm.runInContext('(async()=>{'+source+'})()',context);
 return {elements,input:tracked,axes,get paddle(){return real?.value(0)},frame:t=>frame(t),get starts(){return starts},get pauses(){return pauses},set state(v){state=v},global(type,event={}){for(const fn of listeners.get(type)||[])fn(event)}};
}
test('focused action buttons retain native Enter/Space activation without game hotkeys',async()=>{
 const h=await harness();
 for(const id of ['start','pause'])for(const key of ['Enter',' ']){
  const e=h.elements.get(id).emit('keydown',{key,repeat:false});
  assert.equal(e.defaultPrevented,undefined);assert.equal(h.starts,0);assert.equal(h.pauses,0);
 }
 // The browser owns synthesis of these native button clicks.
 h.elements.get('start').emit('click');assert.equal(h.starts,1);
 h.elements.get('pause').emit('click');assert.equal(h.pauses,1);
});
test('movement buttons support held Enter and Space without restarting or pausing',async()=>{
 const h=await harness(),up=h.elements.get('up'),down=h.elements.get('down');
 assert.equal(up.emit('keydown',{key:'Enter',repeat:false}).defaultPrevented,true);
 assert.equal(h.input.axis(),-1);
 down.emit('keydown',{key:' ',repeat:false});assert.equal(h.input.axis(),0);
 h.global('keyup',{key:'Enter'});assert.equal(h.input.axis(),1);
 h.global('keyup',{key:' '});assert.equal(h.input.axis(),0);
 assert.equal(h.starts,0);assert.equal(h.pauses,0);
});
test('button blur releases its own keys, not simultaneous physical or pointer movement',async()=>{
 const h=await harness(),up=h.elements.get('up');
 h.elements.get('court').emit('keydown',{key:'w',repeat:false});
 up.emit('keydown',{key:'Enter',repeat:false});up.emit('pointerdown',{pointerId:7});
 up.emit('blur');assert.equal(h.input.axis(),-1);
 h.global('keyup',{key:'w'});assert.equal(h.input.axis(),-1);
 up.emit('pointercancel',{pointerId:7});assert.equal(h.input.axis(),0);
});
test('suspension clears button keys; repeat cannot rearm and paused presses are ignored',async()=>{
 const h=await harness(),up=h.elements.get('up');
 up.emit('keydown',{key:' ',repeat:false});h.global('blur');assert.equal(h.input.axis(),0);
 h.state=1;up.emit('keydown',{key:' ',repeat:true});assert.equal(h.input.axis(),0);
 h.state=2;up.emit('keydown',{key:'Enter',repeat:false});assert.equal(h.input.axis(),0);
});
test('court hotkeys still work, while editable controls retain their keys',async()=>{
 const h=await harness(),court=h.elements.get('court');
 court.emit('keydown',{key:' ',repeat:false});assert.equal(h.pauses,1);
 court.emit('keydown',{key:'Enter',repeat:false});assert.equal(h.pauses,2);
 court.emit('keydown',{key:'r',repeat:false});assert.equal(h.starts,1);
 const event={key:' ',repeat:false,target:{closest:()=>({})},preventDefault(){throw Error('must not cancel')}};
 h.global('keydown',event);assert.equal(h.pauses,2);
});

test('movement shortcuts still work after clicking Start leaves a button focused',async()=>{
 const h=await harness(),start=h.elements.get('start');
 start.emit('click');start.emit('keydown',{key:'w',repeat:false});
 assert.equal(h.input.axis(),-1);
 h.global('keyup',{key:'w'});assert.equal(h.input.axis(),0);
});

for (const source of ['keyboard','pointer']) {
 async function assertQuickTap(realBytes) {
  const h=await harness(realBytes),up=h.elements.get('up'),court=h.elements.get('court');
  if(realBytes) h.elements.get('start').emit('click');
  h.frame(100);
  const before=h.paddle;
  if(source==='keyboard'){
   court.emit('keydown',{key:'w',repeat:false});
   court.emit('keyup',{key:'w'});
  }else{
   up.emit('pointerdown',{pointerId:1});up.emit('pointerup',{pointerId:1});
   up.emit('lostpointercapture',{pointerId:1});
  }
  h.frame(104);assert.deepEqual(h.axes,[],source);
  if(realBytes) assert.equal(h.paddle,before);
  h.frame(110);assert.deepEqual(h.axes,[-1],source);
  if(realBytes) assert.ok(h.paddle<before);
  const after=h.paddle;
  h.frame(150);assert.ok(h.axes.length>2,source);
  assert.ok(h.axes.slice(1).every(axis=>axis===0),source);
  if(realBytes) assert.equal(h.paddle,after);
 }
 test(`quick ${source} tap reaches only the first WASM step`, () => assertQuickTap());
 test(`compiled WASM: quick ${source} tap moves the paddle once`, {skip: !process.env.PONG_WASM}, async () => {
  await assertQuickTap(await readFile(process.env.PONG_WASM));
 });
}
