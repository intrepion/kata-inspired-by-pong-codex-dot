param([ValidateSet('yew','leptos','dioxus')][string]$Name='yew')
$ErrorActionPreference='Stop'
$root=Split-Path $PSScriptRoot -Parent
$target=if($env:CARGO_TARGET_DIR){$env:CARGO_TARGET_DIR}else{Join-Path $root 'target-web-frameworks'}
$bindgen=if($env:WASM_BINDGEN){$env:WASM_BINDGEN}else{'wasm-bindgen'}
$argsList=@('build','--locked','--manifest-path',"$root/experiments/$Name/Cargo.toml",'--target','wasm32-unknown-unknown','--target-dir',$target,'-j','2')
if($Name -eq 'dioxus'){$argsList+=@('--no-default-features','--features','web')}
& cargo @argsList
if($LASTEXITCODE -ne 0){throw 'Rust WASM build failed'}
$dist=Join-Path $root "experiments/$Name/dist"
New-Item -ItemType Directory -Force $dist | Out-Null
& $bindgen "$target/wasm32-unknown-unknown/debug/pong-$Name.wasm" --target web --out-dir $dist --out-name app
if($LASTEXITCODE -ne 0){throw 'wasm-bindgen failed'}
[IO.File]::WriteAllText("$dist/index.html",'<!doctype html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Pong '+$Name+'</title></head><body><div id="main"></div><script type="module" src="./boot.js"></script></body></html>')
[IO.File]::WriteAllText("$dist/boot.js","import init from './app.js'; await init();`n")
Write-Output "Built $Name. Serve through http://127.0.0.1, never a file URL."
