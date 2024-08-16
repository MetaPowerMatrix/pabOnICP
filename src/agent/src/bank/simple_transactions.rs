use std::time::Duration;

use ethers::{
    prelude::{Address, LocalWallet, Middleware, Provider, TransactionRequest, U256},
    utils::Ganache,
};
use eyre::{ContextCompat, Result};
use hex::ToHex;

pub async fn pay_with_usdt_use_ethereum(from_address: String, to_address_hex: String, amount_input: f64) -> Result<()> {
    // Spawn a ganache instance
    let mnemonic = "gas monster ski craft below illegal discover limit dog bundle bus artefact";
    let ganache = Ganache::new().mnemonic(mnemonic).spawn();
    println!("HTTP Endpoint: {}", ganache.endpoint());

    // Get the first wallet managed by ganache
    let wallet: LocalWallet = ganache.keys()[0].clone().into();
    // let first_address = wallet.address();
    let first_address_hex = from_address;
    let first_address = first_address_hex.parse::<Address>()?;
    println!(
        "Wallet first address: {}",
        first_address.encode_hex::<String>()
    );

    // A provider is an Ethereum JsonRPC client
    let provider = Provider::try_from(ganache.endpoint())?.interval(Duration::from_millis(10));

    // Query the balance of our account
    let first_balance = provider.get_balance(first_address, None).await?;
    println!("Wallet first address balance: {}", first_balance);

    // Create a transaction to transfer 1000 wei to `other_address`
    let to_address = to_address_hex.parse::<Address>()?;
    let amount_transfer = (amount_input * 10000000.0).floor() as u64;
    let tx = TransactionRequest::pay(to_address, U256::from(amount_transfer)).from(first_address);
    // Send the transaction and wait for receipt
    let receipt = provider
        .send_transaction(tx, None)
        .await?
        .log_msg("Pending transfer")
        .confirmations(1) // number of confirmations required
        .await?
        .context("Missing receipt")?;

    println!(
        "TX mined in block {}",
        receipt.block_number.context("Can not get block number")?
    );
    println!(
        "Balance of {} {}",
        to_address,
        provider.get_balance(to_address, None).await?
    );

    Ok(())
}
