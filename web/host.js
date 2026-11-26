'use strict';
(() => {
  const panel=document.querySelector('#controls'), pause=document.querySelector('#pause');
  const active=new Map(), edges=new Uint32Array(4), slots=[null,null,null,null];
  let lastScene='', lastParty=0, lastLang=-1;
  const keys=['game-hub.save.v1','game-hub.backup.v1'], encoder=new TextEncoder(), decoder=new TextDecoder();
  const sceneNames=['hub','tutorial','playing','paused','settings','end'];
  function refreshPads() {
    let devices=[];
    try { devices=Array.from(navigator.getGamepads?.() ?? []).filter(p=>p && p.connected && p.mapping==='standard'); } catch {}
    for(let i=0;i<4;i++) if(!devices.some(p=>p.index===slots[i])) slots[i]=null;
    for(const p of devices) if(!slots.includes(p.index) && slots.includes(null)) slots[slots.indexOf(null)]=p.index;
    return slots.map(id=>devices.find(p=>p.index===id));
  }
  let pads=[];
  function buttons(p) {
    if(p===0) pads=refreshPads();
    let bits=0;
    for(const v of active.values()) if(v.player===p) bits|=v.bit;
    const gp=pads[p];
    if(gp) {
      const on=i=>gp.buttons[i]?.pressed,x=gp.axes[0]||0,y=gp.axes[1]||0;
      [on(12)||y<-.4,on(13)||y>.4,on(14)||x<-.4,on(15)||x>.4,on(0),on(1),on(9)].forEach((v,i)=>{if(v)bits|=1<<i;});
    }
    return bits;
  }
  function makePads(n) {
    panel.replaceChildren();
    for(let player=0;player<n;player++){
      const group=document.createElement('div');group.className='pad p'+player;
      const label=document.createElement('strong');label.textContent='P'+(player+1);group.append(label);
      for(const [symbol,index] of [['←',2],['↑',0],['A',4],['→',3],['↓',1],['B',5]]){
        const b=document.createElement('button');b.textContent=symbol;b.setAttribute('aria-label',label.textContent+' '+symbol);
        b.onpointerdown=e=>{e.preventDefault(); b.setPointerCapture(e.pointerId);const bit=1<<index;
          active.set(e.pointerId,{player,bit});edges[player]|=bit;};
        b.onpointerup=b.onpointercancel=b.onlostpointercapture=e=>{active.delete(e.pointerId);};group.append(b);
      } panel.append(group);
    }
  }
  function syncScene(id,n,en) {
    const name=sceneNames[id];if(name===lastScene&&n===lastParty&&en===lastLang)return;
    if(name!==lastScene){active.clear();edges.fill(0);}
    if(n!==lastParty)makePads(n);
    lastScene=name;lastParty=n;lastLang=en;document.body.dataset.scene=name;
    document.body.dataset.players=String(n);panel.hidden=name!=='playing';pause.hidden=!['playing','paused'].includes(name);
    pause.textContent=name==='paused'?(en?'Resume':'Tiếp tục'):(en?'Pause':'Tạm dừng');
    window.dispatchEvent(new Event('resize'));
  }
  miniquad_add_plugin({name:'gamehub',version:1,register_plugin(imports){Object.assign(imports.env,{
    hub_buttons:buttons,hub_pad_count:()=>pads.filter(Boolean).length,
    hub_edges(p){const n=edges[p];edges[p]=0;return n;},hub_scene:syncScene,
    hub_read(slot,addr,cap){try{const s=localStorage.getItem(keys[slot]);if(s===null)return -1;if(s.length>65536)return -2;
      const bytes=encoder.encode(s);if(addr){if(bytes.length>cap)return -2;new Uint8Array(wasm_memory.buffer,addr,bytes.length).set(bytes);}return bytes.length;}catch{return -2;}},
    hub_write(slot,addr,len){try{if(len>65536||!keys[slot])return 0;localStorage.setItem(keys[slot],decoder.decode(new Uint8Array(wasm_memory.buffer,addr,len)));return 1;}catch{return 0;}}
  });}});
  const policy=document.querySelector('#policy');pause.onclick=()=>{edges[0]|=64;};
  document.querySelector('#privacy').onclick=()=>{if(lastScene==='playing')edges[0]|=64;policy.showModal();};
  document.querySelector('#close-policy').onclick=()=>{policy.close();canvas.focus();};
  document.addEventListener('visibilitychange',()=>{if(document.hidden){active.clear();if(lastScene==='playing')edges[0]|=64;}});
  document.querySelector('#start').onclick=()=>{
    const btn=document.querySelector('#start');btn.disabled=true;document.querySelector('#status').textContent='Đang tải / Loading…';
    if(!window.WebAssembly || !window.WebGLRenderingContext){document.querySelector('#status').textContent='Cần WebGL và WebAssembly / Update your browser or Android System WebView.';return;}
    load(typeof GAME_HUB_WASM==='undefined'?'game-hub.wasm':URL.createObjectURL(new Blob([Uint8Array.from(atob(GAME_HUB_WASM),c=>c.charCodeAt(0))],{type:'application/wasm'})));
    const timer=setInterval(()=>{if(lastScene){clearInterval(timer);document.querySelector('#welcome').hidden=true;canvas.focus();}},50);
    setTimeout(()=>{if(!lastScene){clearInterval(timer);document.querySelector('#status').textContent='Không mở được game. Hãy tải lại hoặc cập nhật trình duyệt / WebView.';}},30000);
  };
})();

window.gameHubBack = () => {
  const modal=document.querySelector('#policy');
  if(modal.open){modal.close();return true;}
  const active=document.body.dataset.scene;
  if(!active||active==='hub')return false;
  const key=active==='paused'?'Backspace':'Escape';
  const view=document.querySelector('#glcanvas');
  view.dispatchEvent(new KeyboardEvent('keydown',{code:key,key,bubbles:true}));
  setTimeout(()=>view.dispatchEvent(new KeyboardEvent('keyup',{code:key,key,bubbles:true})),65);
  return true;
};
