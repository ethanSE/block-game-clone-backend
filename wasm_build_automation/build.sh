#!/bin/sh
#run from project root

# generates wasm package at /pkg
wasm-pack build --target web && 
# generates typescript types from rust types for rust-ts interop
# places in /pkg/types
cargo test -q && 
# generates index.ts file 
cd wasm_build_automation/generate_index && cargo run &&
# include types folder in package.json
cd ../.. && node wasm_build_automation/add_types.js