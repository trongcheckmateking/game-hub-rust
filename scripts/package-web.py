"""Assemble the web edition. The portable file also opens directly from disk."""
from pathlib import Path
import base64, hashlib, shutil
root=Path(__file__).resolve().parents[1]
out=root/'dist/web'
out.mkdir(parents=True,exist_ok=True)
shutil.copytree(root/'web',out,dirs_exist_ok=True)
wasm=root/'target/wasm32-unknown-unknown/release/game-hub.wasm'
shutil.copy2(wasm,out/'game-hub.wasm')
html=(out/'index.html').read_text()
scripts=[]
for name in ['vendor/mq_js_bundle.js','host.js']:
    js=(out/name).read_text()
    if name=='host.js':
        js="const GAME_HUB_WASM='"+base64.b64encode(wasm.read_bytes()).decode()+"';\n"+js
    html=html.replace(f'<script src="{name}"></script>','<script>'+js+'</script>')
    scripts.append("'sha256-"+base64.b64encode(hashlib.sha256(js.encode()).digest()).decode()+"'")
css=(out/'hub.css').read_text()
style="'sha256-"+base64.b64encode(hashlib.sha256(css.encode()).digest()).decode()+"'"
html=html.replace('<link rel="stylesheet" href="hub.css">','<style>'+css+'</style>').replace("script-src 'self'",'script-src '+' '.join(scripts)).replace("style-src 'self'",'style-src '+style).replace("connect-src 'self'","connect-src blob:")
(root/'dist/GAME-HUB-Offline.html').write_text(html)
