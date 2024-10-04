all: matrix agent
.PHONY: all

agent:
	cargo build --target wasm32-unknown-unknown --package agent_canister --release
	wasm-opt -Os --enable-simd --enable-bulk-memory   -o ./target/wasm32-unknown-unknown/release/agent_canister_opt.wasm  \
        ./target/wasm32-unknown-unknown/release/agent_canister.wasm 

# ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/agent_canister.wasm -o ./target/wasm32-unknown-unknown/release/agent_canister_opt.wasm

matrix:
	cargo build --target wasm32-unknown-unknown --package matrix_canister --release
	wasm-opt -Os --enable-simd --enable-bulk-memory   -o ./target/wasm32-unknown-unknown/release/matrix_canister_opt.wasm  \
        ./target/wasm32-unknown-unknown/release/matrix_canister.wasm 

pato:
	cargo build --target wasm32-unknown-unknown --package pato_canister --release
	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/pato_canister.wasm -o ./target/wasm32-unknown-unknown/release/pato_canister_opt.wasm
