all: matrix agent
.PHONY: all

agent:
	cargo build --target wasm32-unknown-unknown --package agent_canister --release
	wasm-opt -Os --enable-simd --enable-bulk-memory   -o ./target/wasm32-unknown-unknown/release/agent_canister_opt.wasm  \
        ./target/wasm32-unknown-unknown/release/agent_canister.wasm
	candid-extractor ./target/wasm32-unknown-unknown/release/agent_canister_opt.wasm > ./src/agent/agent_canister.did

matrix:
	cargo build --target wasm32-unknown-unknown --package matrix_canister --release
	wasm-opt -Os --enable-simd --enable-bulk-memory   -o ./target/wasm32-unknown-unknown/release/matrix_canister_opt.wasm  \
        ./target/wasm32-unknown-unknown/release/matrix_canister.wasm 
	candid-extractor ./target/wasm32-unknown-unknown/release/matrix_canister_opt.wasm > ./src/matrix/matrix_canister.did

battery:
	cargo build --target wasm32-unknown-unknown --package battery_canister --release
	wasm-opt -Os --enable-simd --enable-bulk-memory   -o ./target/wasm32-unknown-unknown/release/battery_canister_opt.wasm  \
        ./target/wasm32-unknown-unknown/release/battery_canister.wasm 
	candid-extractor target/wasm32-unknown-unknown/release/battery_canister_opt.wasm > src/battery/battery_canister.did
