#[cfg(test)]
mod tests {
    use crate::blockchain::{block::Block, blockchain::Blockchain, file_infor::BlockInformation};
    use datetime::Instant;
    use rand::{distributions::Alphanumeric, rngs::ThreadRng, thread_rng, Rng};

    #[test]
    fn test_replacement() {
        let mut blockchain = gen_blockchain(3);
        let mut csprng = ThreadRng::default();
        let account = ed25519_dalek::Keypair::generate(&mut csprng);

        let linked_uri = get_random(8);
        let timestamp = Instant::now().seconds();
        let signature = ed25519_dalek::ExpandedSecretKey::from(&account.secret).sign(
            (linked_uri.clone() + &timestamp.to_string() + &0.to_string()).as_bytes(),
            &account.public,
        );

        let block4 = Block::new(
            blockchain.0.len() as u128,
            blockchain.0.last().unwrap().hash.clone(),
            BlockInformation {
                data: Some(get_random(8)),
                linked_uri,
                creator: account.public.to_bytes().to_vec(),
                version: String::from("0.0.0"),
                file_type: crate::blockchain::file_infor::FileType::DataStore,
                signature: signature.to_bytes().to_vec(),
                timestamp: timestamp as i128,
                tokens_transferred: 0,
                to_acct_id: String::from("testing shit"),
            },
        );

        let mut replacement_chain = blockchain.clone();
        replacement_chain.add_unverified_block(block4);

        println!("{}", replacement_chain.0.len());

        let success = blockchain.replace_chain(replacement_chain.0);

        assert_eq!(success, true);
    }

    fn gen_blockchain(length: u8) -> Blockchain {
        let mut blockchain = Blockchain::new();
        let mut csprng = ThreadRng::default();
        for _ in 1..length {
            let account = ed25519_dalek::Keypair::generate(&mut csprng);
            let linked_uri = get_random(8);
            let timestamp = Instant::now().seconds();
            let signature = ed25519_dalek::ExpandedSecretKey::from(&account.secret).sign(
                (linked_uri.clone() + &timestamp.to_string() + &0.to_string()).as_bytes(),
                &account.public,
            );
            let block = Block::new(
                blockchain.0.len() as u128,
                blockchain.0.last().unwrap().hash.clone(),
                BlockInformation {
                    data: Some(get_random(8)),
                    linked_uri: linked_uri.clone(),
                    creator: account.public.to_bytes().to_vec(),
                    version: String::from("0.0.0"),
                    file_type: crate::blockchain::file_infor::FileType::DataStore,
                    signature: signature.to_bytes().to_vec(),
                    timestamp: timestamp as i128,
                    tokens_transferred: 0,
                    to_acct_id: String::from("testing shit"),
                },
            );
            blockchain.add_unverified_block(block);
        }
        return blockchain;
    }

    fn get_random(amt: usize) -> String {
        return thread_rng()
            .sample_iter(&Alphanumeric)
            .take(amt)
            .map(char::from)
            .collect::<String>();
    }
}
