import wallet from "../wallet/dev-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/BrebUfRk4FVmQ1duqa7mGxqbr2jC3f6k8m5PZuZu7eMu"
        const metadata = {
            name: "My NFT",
            symbol: "DGC",
            description: "My first NFT",
            image,
            attributes: [
                {trait_type: 'trait1', value: '20'},
                {trait_type: 'trait2', value: '30'},
                {trait_type: 'trait3', value: '40'},
                {trait_type: 'trait4', value: '10'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: [
                {
                    address: keypair.publicKey,
                    share: 100
                }
            ]
        };

        const genericFile = createGenericFile(JSON.stringify(metadata), 'NFT-metadata', {contentType: 'application/json'});
        const myUri = await umi.uploader.uploadJson(genericFile);
        console.log("Your metadata URI: ", myUri);

        // Create a generic file
        
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
