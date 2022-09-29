#!/bin/sh
cp -r /lib_fl/* /proj/lib_fl/
echo "$(cat /proj/lib_fl/Cargo.toml | dasel put string -r toml 'package.name' 'lib_fl')" > /proj/lib_fl/Cargo.toml
cargo wasi build --release
cp /proj/target/wasm32-wasi/release/wrapper.wasm /out_wasm/code.wasm
