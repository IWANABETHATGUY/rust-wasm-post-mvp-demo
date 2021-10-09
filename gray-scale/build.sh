#!/bin/bash
wasm-pack build --target web

cp target/wasm32-unknown-unknown/release/gray_scale.wasm ./www/