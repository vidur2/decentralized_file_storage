#[cfg(test)]
mod tests {
    use crate::blockchain::{block::Block, blockchain::Blockchain, file_infor::BlockInformation};
    use datetime::Instant;
    use rand::{rngs::ThreadRng, thread_rng, distributions::Alphanumeric, Rng};

    #[test]
    fn test() {
        let mut csprng = ThreadRng::default();
        let mut blockchain = Blockchain::new();
        let account = ed25519_dalek::Keypair::generate(&mut csprng);
        let linked_uri = get_random(8);
        let timestamp = Instant::now().seconds();
        let signature = ed25519_dalek::ExpandedSecretKey::from(&account.secret).sign((linked_uri.clone() + &timestamp.to_string() + &0.to_string()).as_bytes(), &account.public);
        println!("{}", signature);
        let block1 = Block::new(blockchain.0.len() as u128, blockchain.0.last().unwrap().hash.clone(), BlockInformation {
            data: Some(get_random(8)),
            linked_uri: linked_uri.clone(),
            creator: account.public.to_bytes(),
            version: String::from("0.0.0"),
            file_type: crate::blockchain::file_infor::FileType::DataStore,
            signature: signature.to_bytes().to_vec(),
            timestamp: timestamp as i128,
            tokens_transferred: 0,
            to_acct_id: String::from("testing shit"),
        });

        blockchain.add_unverified_block(block1);

        let linked_uri = get_random(8);
        let timestamp = Instant::now().seconds();
        let signature = ed25519_dalek::ExpandedSecretKey::from(&account.secret).sign((linked_uri.clone() + &timestamp.to_string() + &0.to_string()).as_bytes(), &account.public);

        let block2 = Block::new(blockchain.0.len() as u128, blockchain.0.last().unwrap().hash.clone(), BlockInformation {
            data: Some(get_random(8)),
            linked_uri: linked_uri.clone(),
            creator: account.public.to_bytes(),
            version: String::from("0.0.0"),
            file_type: crate::blockchain::file_infor::FileType::DataStore,
            signature: signature.to_bytes().to_vec(),
            timestamp: timestamp as i128,
            tokens_transferred: 0,
            to_acct_id: String::from("testing shit"),
        });

        blockchain.add_unverified_block(block2);

        let linked_uri = get_random(8);
        let timestamp = Instant::now().seconds();
        let signature = ed25519_dalek::ExpandedSecretKey::from(&account.secret).sign((linked_uri.clone() + &timestamp.to_string() + &0.to_string()).as_bytes(), &account.public);

        let block3 = Block::new(blockchain.0.len() as u128, blockchain.0.last().unwrap().hash.clone(), BlockInformation {
            data: Some(get_random(8)),
            linked_uri: linked_uri.clone(),
            creator: account.public.to_bytes(),
            version: String::from("0.0.0"),
            file_type: crate::blockchain::file_infor::FileType::DataStore,
            signature: signature.to_bytes().to_vec(),
            timestamp: timestamp as i128,
            tokens_transferred: 0,
            to_acct_id: String::from("testing shit"),
        });

        blockchain.add_unverified_block(block3);

        let linked_uri = get_random(8);
        let timestamp = Instant::now().seconds();
        let signature = ed25519_dalek::ExpandedSecretKey::from(&account.secret).sign((linked_uri.clone() + &timestamp.to_string() + &0.to_string()).as_bytes(), &account.public);

        let block4 = Block::new(blockchain.0.len() as u128, blockchain.0.last().unwrap().hash.clone(), BlockInformation {
            data: Some(get_random(8)),
            linked_uri,
            creator: account.public.to_bytes(),
            version: String::from("0.0.0"),
            file_type: crate::blockchain::file_infor::FileType::DataStore,
            signature: signature.to_bytes().to_vec(),
            timestamp: timestamp as i128,
            tokens_transferred: 0,
            to_acct_id: String::from("testing shit"),
        });

        let mut replacement_chain = blockchain.clone();
        replacement_chain.add_unverified_block(block4);

        println!("{}", replacement_chain.0.len());

        let success = blockchain.replace_chain(replacement_chain.0);

        assert_eq!(success, true);
    }

    fn get_random(amt: usize) -> String {
        return thread_rng()
            .sample_iter(&Alphanumeric)
            .take(amt)
            .map(char::from)
            .collect::<String>()
    }
}
