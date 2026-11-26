const {chromium}=require('playwright-core');
const {pathToFileURL}=require('url');
const path=require('path');
(async()=>{
 const mobile=process.argv.includes('mobile');
 const b=await chromium.launch({executablePath:process.env.CHROME_BIN || '/usr/bin/google-chrome',headless:true,args:['--no-sandbox','--use-gl=angle','--use-angle=swiftshader','--enable-unsafe-swiftshader']});
 try {
 const ctx=await b.newContext({viewport:mobile?{width:915,height:412}:{width:1200,height:820},hasTouch:mobile,isMobile:mobile,deviceScaleFactor:1});
 const page=await ctx.newPage();const issues=[];page.on('pageerror',e=>issues.push(e.message));page.on('console',m=>{if(['error','warning'].includes(m.type()))issues.push(m.text().slice(0,260));});
 await ctx.setOffline(true);await page.goto(pathToFileURL(path.resolve('dist/GAME-HUB-Offline.html')).href);
 await page.locator('#start').click();try{await page.waitForFunction(()=>document.body.dataset.scene==='hub',{},{timeout:16000});}catch(e){throw Error('boot '+JSON.stringify(issues)+' status '+await page.locator('#status').textContent());}
 async function click(x,y){const r=await page.locator('#glcanvas').boundingBox();const s=Math.min(r.width/1120,r.height/720);const px=r.x+(r.width-1120*s)/2+x*s,py=r.y+(r.height-720*s)/2+y*s;
 if(mobile) {const cdp=await ctx.newCDPSession(page);await cdp.send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{x:px,y:py}]});await page.waitForTimeout(85);await cdp.send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});await cdp.detach();}else await page.mouse.click(px,py,{delay:75});await page.waitForTimeout(150);}
 async function scene(s){await page.waitForFunction(v=>document.body.dataset.scene===v,s,{timeout:6000});}
 await page.waitForTimeout(220);
 await page.screenshot({path:'target/hub-'+(mobile?'mobile':'web')+'.png'});
 await click(140,196); // solo
 await click(200,355);await scene('tutorial');await click(153,621);await scene('playing'); // Ur
 if(mobile){await page.locator('.p0 button').filter({hasText:/^A$/}).tap();}else await page.keyboard.press('KeyF');
 await page.waitForTimeout(350);await page.screenshot({path:'target/ur-'+(mobile?'mobile':'web')+'.png'});
 await page.locator('#pause').click();await scene('paused');await click(498,495);await scene('hub');
 await click(274,196); // 3P
 await click(202,354);await scene('tutorial');await click(153,621);await scene('playing');
 if(mobile && await page.locator('#controls .pad').count()!==3) throw Error('3P touch pads missing');
 await page.screenshot({path:'target/stones-'+(mobile?'mobile':'web')+'.png'});
 if(mobile){await page.locator('.p0 button').filter({hasText:/^A$/}).tap();await page.locator('.p1 button').filter({hasText:/^A$/}).tap();}else{await page.keyboard.press('KeyF');await page.keyboard.press('Enter');}
 await page.locator('#privacy').click();await page.waitForFunction(()=>document.querySelector('#policy').open);await page.locator('#close-policy').click();
 await scene('paused');await click(498,495);await scene('hub');
 await click(72,196); // All -> page 4, second card: Air Hockey
 for(let i=0;i<3;i++)await click(987,640);
 await click(553,354);await scene('tutorial');await click(153,621);await scene('playing');
 if(mobile && await page.locator('#controls .pad').count()!==2)throw Error('2P hockey pads missing');
 if(mobile){await page.locator('.p0 button').filter({hasText:'→'}).tap();}
 else {await page.keyboard.down('KeyD');await page.waitForTimeout(350);await page.keyboard.up('KeyD');}
 await page.waitForTimeout(500);
 if(mobile){
   const layout=await page.evaluate(()=>{
     const canvas=document.querySelector('#glcanvas'),box=canvas.getBoundingClientRect(),controls=document.querySelector('#controls').getBoundingClientRect();
     return {top:box.top,bottom:box.bottom,padTop:controls.top,padBottom:controls.bottom,screen:innerHeight,scroll:scrollY,buffer:canvas.height,css:canvas.clientHeight};
   });
   if(Math.abs(layout.top)>1 || layout.scroll>1 || Math.abs(layout.bottom-layout.padTop)>2 || layout.padBottom>layout.screen+2 || Math.abs(layout.buffer-layout.css)>1)
     throw Error('Canvas or touch controls clipped: '+JSON.stringify(layout));
 }
 await page.screenshot({path:'target/hockey-'+(mobile?'mobile':'web')+'.png'});
 console.log('offline',mobile?'mobile':'desktop','Ur 1P, stones 3P, Air Hockey 2P, pause, privacy passed',issues);
 }finally{await b.close();}
})().catch(e=>{console.error(e.message);process.exitCode=1;});
