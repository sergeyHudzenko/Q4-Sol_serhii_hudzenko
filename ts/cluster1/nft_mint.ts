import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wallet/dev-wallet.json"
import base58 from "bs58";
import { create } from 'domain';

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const name = "My NFT";
const uri = "https://devnet.irys.xyz/BrebUfRk4FVmQ1duqa7mGxqbr2jC3f6k8m5PZuZu7eMu";
const symbol = "DGC";

const mint = generateSigner(umi);
const sellerFeeBasisPoints = percentAmount(0, 2);

(async () => {
    let tx = createNft(umi, { mint, name, symbol, uri, sellerFeeBasisPoints});
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();