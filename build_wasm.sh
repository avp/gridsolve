#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

wasm-pack build --release --target web crates/gridsolve_wasm --out-dir "$PWD/www/pkg"
mkdir -p www/dist/
cp www/pkg/gridsolve_wasm_bg.wasm www/dist/
