const test=require('node:test'),assert=require('node:assert/strict'),fs=require('node:fs'),vm=require('node:vm'),path=require('node:path');
function fixture(file) {
 let nextFrame, playerY, rect={top:0,height:500};
 class Element { constructor(){this.listeners={};this.firstChild={textContent:''};} addEventListener(n,f){(this.listeners[n]??=[]).push(f);} emit(n,e={}){for(const f of this.listeners[n]??[])f({preventDefault(){},stopPropagation(){},...e});} setPointerCapture(){} getBoundingClientRect(){return rect;} }
 class Button extends Element {}
 class Canvas extends Element { getContext(){return {fillRect(x,y){if(x===32)playerY=y;},setLineDash(){},beginPath(){},moveTo(){},lineTo(){},stroke(){},arc(){},fill(){}};} }
 const names=['court','overlay','message','hint','start','pause','status','you','cpu','up','down'];
 const els=Object.fromEntries(names.map(n=>[n,n==='court'?new Canvas():['start','pause','up','down'].includes(n)?new Button():new Element()]));
 const doc=new Element();doc.querySelector=s=>els[s.slice(1)];doc.hidden=false;
 vm.runInNewContext(fs.readFileSync(file,'utf8'),{document:doc,window:new Element(),HTMLElement:Element,HTMLButtonElement:Button,HTMLCanvasElement:Canvas,URLSearchParams,location:{search:''},requestAnimationFrame:f=>{nextFrame=f;},console});
 const frame=t=>nextFrame(t), move=(y)=>els.court.emit('pointermove',{pointerType:'touch',pointerId:1,buttons:1,clientY:y});
 els.start.emit('click'); frame(0); els.court.emit('pointerdown',{pointerType:'touch',pointerId:1});
 return {setRect:r=>{rect=r;},move,frame,get y(){return playerY;}};
}
test('candidate ignores zero height and continues normal captured drag',()=>{
 const f=fixture(path.join(__dirname,'../game.js')),before=f.y;
 f.setRect({top:0,height:0});f.move(0);f.frame(10);assert.equal(f.y,before);
 f.setRect({top:0,height:500});f.move(350);f.frame(20);assert.equal(f.y,305);
});
test('invalid geometry and coordinates do not poison the paddle',()=>{
 const f=fixture(path.join(__dirname,'../game.js')),before=f.y;
 for(const [rect,y] of [[{top:0,height:-1},0],[{top:NaN,height:500},50],[{top:0,height:Infinity},50],[{top:0,height:500},NaN]]) {
  f.setRect(rect);f.move(y);f.frame(0);assert.equal(f.y,before);
 }
 f.setRect({top:0,height:500});f.move(100);f.frame(0);assert.equal(f.y,55);
});
