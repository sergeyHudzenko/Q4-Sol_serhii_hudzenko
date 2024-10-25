use bs58;
use std::io;

fn base58_to_wallet() {
    println!("Enter your base58 string:");
    let mut base58 = String::new();
    io::stdin().read_line(&mut base58).unwrap();
    let base58 = base58.trim();  // Remove newline from input

    match bs58::decode(base58).into_vec() {
        Ok(wallet) => println!("Decoded wallet format: {:?}", wallet),
        Err(err) => println!("Error decoding base58: {}", err),
    }
}

fn wallet_to_base58() {
    let wallet: Vec<u8> = vec![233, 197, 48, 207, 206, 0, 4, 85, 85, 79, 58, 210, 231, 207, 34, 207, 26, 126, 62, 22, 227, 197, 107, 149, 161, 201, 190, 255, 48, 31, 170, 249, 89, 179, 105, 193, 198, 186, 172, 77, 163, 79, 157, 152, 15, 55, 151, 32, 251, 69, 20, 254, 67, 209, 124, 172, 145, 40, 161, 1, 122, 2, 124, 25];

    let base58 = bs58::encode(wallet).into_string();
    println!("Encoded base58 format: {:?}", base58);
}

fn main() {
    // Call the functions here
    base58_to_wallet();
    wallet_to_base58();
}
