all: matrix agent
.PHONY: all

agent:
	cargo build --release --target=wasm32-wasip1
	wasi2ic ./target/wasm32-wasip1/release/agent_canister.wasm ./target/wasm32-wasip1/release/agent_canister-ic.wasm
	wasm-opt -Os --enable-simd --enable-bulk-memory   -o ./target/wasm32-wasip1/release/agent_canister-ic.wasm ./target/wasm32-wasip1/release/agent_canister-ic-opt.wasm

matrix:
	cargo build --target wasm32-unknown-unknown --package matrix_canister --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/matrix_canister.wasm -o ./target/wasm32-unknown-unknown/release/matrix_canister_opt.wasm

portal:
	cargo build --release --target=wasm32-wasip1
	wasi2ic ./target/wasm32-wasip1/release/portal_canister.wasm ./target/wasm32-wasip1/release/portal_canister-ic.wasm
	wasm-opt -Os --enable-simd --enable-bulk-memory   -o ./target/wasm32-wasip1/release/portal_canister-ic.wasm ./target/wasm32-wasip1/release/portal_canister-ic-opt.wasm

# 	cargo build --target wasm32-unknown-unknown --package portal_canister --release

# 	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/board_canister.wasm -o ./target/wasm32-unknown-unknown/release/board_canister_opt.wasm

# assets:
# 	cargo build --target wasm32-unknown-unknown --package Assets_canister --release

# 	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/Assets_canister.wasm -o ./target/wasm32-unknown-unknown/release/Assets_canister_opt.wasm
