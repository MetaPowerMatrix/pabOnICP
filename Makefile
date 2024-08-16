all: matrix agent portal
.PHONY: all

agent:
	cargo build --target wasm32-unknown-unknown --package agent_canister --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/anderson.wasm -o ./target/wasm32-unknown-unknown/release/anderson_opt.wasm

matrix:
	cargo build --target wasm32-unknown-unknown --package matrix_canister --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/nais_canister.wasm -o ./target/wasm32-unknown-unknown/release/nais_canister_opt.wasm

portal:
	cargo build --target wasm32-unknown-unknown --package portal_canister --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/board_canister.wasm -o ./target/wasm32-unknown-unknown/release/board_canister_opt.wasm

assets:
	cargo build --target wasm32-unknown-unknown --package Assets_canister --release

	ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/Assets_canister.wasm -o ./target/wasm32-unknown-unknown/release/Assets_canister_opt.wasm
