const canvas=document.querySelector('#court'),ctx=canvas.getContext('2d'),status=document.querySelector('#status');
try{
 const response=await fetch('pong_wasm.wasm');if(!response.ok)throw Error('Build the WASM module first (see desktop README).');
 const {instance}=await WebAssembly.instantiate(await response.arrayBuffer(),{}),g=instance.exports;
 g.init(crypto.getRandomValues(new Uint32Array(1))[0]);let last=0,accumulator=0,touch=0;const keys=new Set();
 const start=()=>{g.start();last=0;accumulator=0;},pause=()=>{g.toggle_pause();last=0;accumulator=0;};
 document.querySelector('#start').onclick=start;document.querySelector('#pause').onclick=pause;
 for(const [id,dir]of [['up',-1],['down',1]]){const b=document.getElementById(id);b.onpointerdown=e=>{touch=dir;b.setPointerCapture(e.pointerId)};for(const event of ['pointerup','pointercancel','lostpointercapture'])b.addEventListener(event,()=>touch=0);}
 addEventListener('keydown',e=>{if(['ArrowUp','ArrowDown',' ','Enter'].includes(e.key))e.preventDefault();keys.add(e.key.toLowerCase());if(!e.repeat){if(e.key==='Enter'){if(g.value(6)===2)pause();else if(g.value(6)!==1)start();}if(e.key===' ')pause();if(e.key.toLowerCase()==='r')start();}});addEventListener('keyup',e=>keys.delete(e.key.toLowerCase()));
 function blur(){keys.clear();touch=0;g.pause();last=0;accumulator=0;}addEventListener('blur',blur);document.addEventListener('visibilitychange',()=>{if(document.hidden)blur()});
 function frame(t){if(last){accumulator+=Math.min((t-last)/1000,0.1);while(accumulator>=1/120){g.step(1/120,touch+Number(keys.has('s')||keys.has('arrowdown'))-Number(keys.has('w')||keys.has('arrowup')));accumulator-=1/120;}}last=t;ctx.fillStyle='#153f37';ctx.fillRect(0,0,900,500);ctx.fillStyle='#648474';for(let y=0;y<500;y+=28)ctx.fillRect(449,y,2,12);ctx.fillStyle='#d9ffae';ctx.fillRect(32,g.value(0)-45,12,90);ctx.fillRect(856,g.value(1)-45,12,90);ctx.beginPath();ctx.arc(g.value(2),g.value(3),8,0,Math.PI*2);ctx.fill();status.textContent=`You ${g.value(4)} — ${g.value(5)} CPU · ${['Ready','Playing','Paused','Match over'][g.value(6)]}`;if(g.value(6)!==1){ctx.font='28px system-ui';ctx.textAlign='center';ctx.fillText(['Press Enter or Start','','Paused — Space to resume','Match over — Enter to restart'][g.value(6)],450,250)}requestAnimationFrame(frame)}requestAnimationFrame(frame);
}catch(e){status.textContent=e.message;console.error(e);}
