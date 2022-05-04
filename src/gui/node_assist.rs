use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    blockchain::{block::Block, blockchain::Blockchain},
    http_server::{handle_socket_connection, SharedSocket},
};

use super::{MIDDLEWARE_ADDR_ADD_SELF, MIDDLEWARE_ADDR_GET_BLOCKS, MIDDLEWARE_ADDR_GET_PEERS};

/// Initialization code for the node
/// * Connects to middleware
/// * Connects to all other nodes
///
/// ##  Arguments
/// * `blockchain`: A shared reference to a constructed Blockchain struct
/// * `sockets`: A vector of websockets
pub fn init_node(
    blockchain: crate::blockchain::blockchain::SharedChain,
    sockets: Arc<Mutex<Vec<SharedSocket>>>,
    public_key: String,
) {
    let client = reqwest::blocking::Client::new();
    let resp_peers = reqwest::blocking::get(MIDDLEWARE_ADDR_GET_PEERS)
        .unwrap()
        .text()
        .unwrap();

    let resp = client
        .post(MIDDLEWARE_ADDR_ADD_SELF)
        .body(public_key)
        .send()
        .unwrap()
        .text()
        .unwrap();

    let mut reffed_bc = blockchain.lock().unwrap();

    if &resp == "true" && &resp_peers != "[" {
        let blockchain_str = reqwest::blocking::get(MIDDLEWARE_ADDR_GET_BLOCKS)
            .unwrap()
            .text()
            .unwrap();
        let blockchain_vec: Vec<Block> = serde_json::from_str(&blockchain_str).unwrap();
        *reffed_bc = Blockchain {
            chain: blockchain_vec,
        };

        drop(blockchain_str);
        drop(reffed_bc);

        let parsed_response: Vec<&str> = serde_json::from_str(&resp_peers).unwrap();

        for host in parsed_response.iter() {
            let blockchain = Arc::clone(&blockchain);
            let ws = Arc::new(Mutex::new(
                tungstenite::client::connect(format!("ws://{}", host))
                    .unwrap()
                    .0,
            ));
            let sockets = Arc::clone(&sockets);
            sockets.lock().unwrap().push(Arc::clone(&ws));

            thread::spawn(move || {
                handle_socket_connection(ws, blockchain, sockets);
            });
        }
    } else if &resp == "true" {
        *reffed_bc = Blockchain::new();
    }

    println!("Done!")
}
