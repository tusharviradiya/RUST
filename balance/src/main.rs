// SALXAT3ROTHPXAG6AM272Y4KXL75GSIXEXRYKIOTEJWNG4LREAQSTUUF
// use stellar_sdk::crypto::Keypair;
// use stellar_sdk::sync::Client;
// use stellar_sdk::error::Result;

fn main() {
    // Set up the Stellar client for the testnet
    let client = Sorizon("https://horizon-testnet.stellar.org".to_string());

    // Define your Stellar account's public key
    let account_id = "SALXAT3ROTHPXAG6AM272Y4KXL75GSIXEXRYKIOTEJWNG4LREAQSTUUF";

    // Get the account information from the Stellar blockchain
    match get_account_balance(&client, account_id) {
        Ok(balance) => println!("Balance of account {}: {}", account_id, balance),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_account_balance(client: &Client, account_id: &str) -> Result<String> {
    // Get the account details from the Stellar blockchain
    let account = client.account().account_id(account_id).call()?;

    // Extract the balance from the account response
    for balance in account.balances {
        if balance.asset_type == "native" {
            return Ok(balance.balance);
        }
    }

    Err("Native balance not found".into())
}
