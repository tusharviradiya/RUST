const StellarSdk = require('stellar-sdk');

async function getAccountBalance(accountPublicKey) {
    // Horizon server for the testnet
    const server = new StellarSdk.Horizon.Server('https://horizon-testnet.stellar.org');

    try {
        // Load the account details
        const account = await server.loadAccount(accountPublicKey);

        // Find the native balance
        let nativeBalance;
        account.balances.forEach(balance => {
            if (balance.asset_type === 'native') {
                nativeBalance = balance.balance;
            }
        });

        return nativeBalance || '0'; // If native balance not found, return 0
    } catch (error) {
        throw new Error('Failed to retrieve account details: ' + error.response.data.extras.result_codes.operations[0]);
    }
}

async function main() {
    // Define the public key of your testnet account
    const accountPublicKey = 'GAAYR2MYXHRFAFY7NKXUKYI3KZZWBPLHMR2OWGPQAJ5RC7QT2V32ZCJS';

    try {
        // Get the account balance
        const balance = await getAccountBalance(accountPublicKey);
        console.log(`Balance of account ${accountPublicKey}: ${balance}`);
    } catch (error) {
        console.error('Error:', error.message);
    }
}

main();
