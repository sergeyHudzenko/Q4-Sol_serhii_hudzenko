mod programs;

use solana_sdk::{message::Message, signature::{Keypair, Signer, read_keypair_file}, pubkey::Pubkey};
use solana_client::rpc_client::RpcClient;
use solana_program::system_instruction::transfer;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;
use solana_program::system_program;


use crate::programs::turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs, UpdateArgs};


const RPC_URL: &str = "https://api.devnet.solana.com";


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("New Solana wallet: {}", kp.pubkey().to_string());
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("../dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => println!("Airdrop successful. Transaction: {}", s),
            Err(e) => println!("Airdrop failed: {}", e),
        }
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("../dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("48NYRA9War1ceLYWx24KFoiin29ibAQ6HcaVpQ9JRryz").unwrap();
        let client = RpcClient::new(RPC_URL);
        let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let balance = client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
        println!("Account balance: {} lamports", balance);
        

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );

        let signature = client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        println!("Transfer successful. Transaction: {}", signature);
    }

    #[test]
    fn empty() {
        let keypair = read_keypair_file("../dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("48NYRA9War1ceLYWx24KFoiin29ibAQ6HcaVpQ9JRryz").unwrap();
        let client = RpcClient::new(RPC_URL);
        let recent_blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");
        let balance = client.get_balance(&keypair.pubkey()).expect("Failed to get balance");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        
        let fee = client.get_fee_for_message(&message).expect("Failed to calculate fee");
        println!("Fee: {}", fee);

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash,
        );

        let signature = client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");
        println!("Transfer successful. Transaction: {}", signature);
        
    }

    #[test]
    fn complete() {
        let client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("../Turbin3-wallet.json").expect("Couldn't find wallet file");

        let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);
        let args = CompleteArgs { github: b"sergeyHudzenko".to_vec() };
        let blockhash = client.get_latest_blockhash().expect("Failed to get recent blockhash");

        let transaction = Turbin3PrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );
        
        let signature = client.send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        
        println!("Enrollment completed. Check your transaction at: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
        
    }
}
