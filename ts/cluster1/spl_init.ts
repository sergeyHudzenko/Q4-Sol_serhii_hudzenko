import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "../wallet/dev-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint account created: 4eAXjMMFJ5T2xCAKsFrJZzqm825sVhiygmjJibZMcqNR

(async () => {
    try {
        // Start here
        // Create mint account
        const mint  = await createMint(connection, keypair, keypair.publicKey, null, 6)
        console.log(`Mint account created: ${mint.toBase58()}`)
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
