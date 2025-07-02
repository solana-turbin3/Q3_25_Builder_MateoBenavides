#[cfg(test)]
mod tests {
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        pubkey::Pubkey,
        system_instruction::transfer,
        transaction::Transaction,
        message::Message,
        instruction::{Instruction, AccountMeta},
        system_program,
    };
    use solana_client::rpc_client::RpcClient;
    use solana_program::hash::hash;
    use std::str::FromStr;
    use std::io::{self, BufRead};
    use bs58;

    const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

   
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        let keypair = read_keypair_file("rustprereq-wallet.json").expect("Couldn't find wallet file");
        
        
        let client = RpcClient::new(RPC_URL);
        
        
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }
            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }
    }

    #[test]
    fn check_balance() {
        let keypair = read_keypair_file("rustprereq-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);
        
        match client.get_balance(&keypair.pubkey()) {
            Ok(balance) => {
                println!("Wallet address: {}", keypair.pubkey().to_string());
                println!("Balance: {} lamports", balance);
                println!("Balance: {} SOL", balance as f64 / 1_000_000_000.0);
            }
            Err(err) => {
                println!("Failed to get balance: {}", err);
            }
        }
    }

    #[test]
    fn transfer_sol() {
        
        let keypair = read_keypair_file("rustprereq-wallet.json").expect("Couldn't find wallet file");
        
       
        let to_pubkey = Pubkey::from_str("RZpLfne8gCzFGLzeHz8ejUUp1Vj1vyBfn4Zk163uNQe").unwrap();
        
 
        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
      
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );
        
   
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn empty_wallet() {
   
        let keypair = read_keypair_file("rustprereq-wallet.json").expect("Couldn't find wallet file");
        
    
        let to_pubkey = Pubkey::from_str("RZpLfne8gCzFGLzeHz8ejUUp1Vj1vyBfn4Zk163uNQe").unwrap();
        
 
        let rpc_client = RpcClient::new(RPC_URL);
        
  
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
      
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");
        
        println!("Current balance: {} lamports ({} SOL)", balance, balance as f64 / 1_000_000_000.0);
        
 
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );
        
      
        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");
        
        println!("Transaction fee: {} lamports", fee);
        println!("Transferring: {} lamports ({} SOL)", balance - fee, (balance - fee) as f64 / 1_000_000_000.0);
        
     
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");
        
        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn submit_rs() {

        let rpc_client = RpcClient::new(RPC_URL);
        
       
        let signer = read_keypair_file("turbin3-wallet.json")
            .expect("Couldn't find Turbin3 wallet file");
        
        println!("Using Turbin3 wallet: {}", signer.pubkey().to_string());
        
      
        let mint = Keypair::new();
        println!("NFT mint address: {}", mint.pubkey().to_string());
        
   
        let turbin3_prereq_program = 
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = 
            Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = 
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();
        
      
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
        println!("Your PDA: {}", prereq_pda.to_string());
        
      
        let authority_seeds = &[b"collection", collection.as_ref()];
        let (authority, _auth_bump) = Pubkey::find_program_address(authority_seeds, &turbin3_prereq_program);
        println!("Authority PDA: {}", authority.to_string());
        
    
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
        
   
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),           // user (signer)
            AccountMeta::new(prereq_pda, false),              // PDA account
            AccountMeta::new(mint.pubkey(), true),            // mint keypair
            AccountMeta::new(collection, false),              // collection
            AccountMeta::new_readonly(authority, false),      // authority PDA
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false), // system program
        ];
        
       
        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
 
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };
        
     
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );
        

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        
        println!(
            "🎉 SUCCESS! Check out your completion NFT here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
        println!("You have successfully completed the Turbin3 Rust prerequisites!");
    }
}