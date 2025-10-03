#!/bin/bash

# crate_name=cargo tree --depth 0 | awk '{print $1;}'
rm -rf ./out
cargo build --release --target wasm32-unknown-unknown
mkdir ./out
wasm-bindgen --out-dir ./out/ --target web --no-typescript --out-name "bevy_quickstart_game" target/wasm32-unknown-unknown/release/bevy_quickstart_game.wasm
cp index.html ./out
mkdir ./out/assets/
cp -r ./assets/* ./out/assets/
zip -rq out.zip out