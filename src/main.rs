#![feature(let_chains)]

use std::{
    sync::{Arc, Mutex},
    thread,
};

use blockchain::{
    block::Block,
    blockchain::{Blockchain, SharedChain},
};
use http_server::SharedSocket;

use crate::http_server::handle_socket_connection;

mod blockchain;
mod http_server;
mod tests;

const MIDDLEWARE_ADDR_GET_BLOCKS: &str = "http://localhost:8080/get_blocks";
const MIDDLEWARE_ADDR_GET_PEERS: &str = "http://localhost:8080/get_peers";
const MIDDLEWARE_ADDR_ADD_SELF: &str = "http://localhost:8080/add_self_as_peer";

fn get_node_pk() -> String {
    println!("Enter your public key here (if you dont have one, just press enter): ");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Could not read");
    if line != "\n" {
        line = line.replace("\n", "");
    } else {
        let mut csprng = rand::prelude::ThreadRng::default();
        let account = ed25519_dalek::Keypair::generate(&mut csprng);

        println!("Your public key is {:?}", account.public.to_bytes());
        println!("Your private key is {:?}", account.secret.to_bytes());
        line = serde_json::to_string(&account.public.to_bytes()).unwrap()
    }
    
    return line
}

/// Initialization code for the node
/// * Connects to middleware
/// * Connects to all other nodes
///
/// ##  Arguments
/// * `blockchain`: A shared reference to a constructed Blockchain struct
/// * `sockets`: A vector of websockets
fn init_node(
    blockchain: crate::blockchain::blockchain::SharedChain,
    sockets: Arc<Mutex<Vec<SharedSocket>>>,
) {
    let public_key = get_node_pk();
    let client = reqwest::blocking::Client::new();
    let resp_peers = reqwest::blocking::get(MIDDLEWARE_ADDR_GET_PEERS)
        .unwrap()
        .text()
        .unwrap();

    let resp = client.post(MIDDLEWARE_ADDR_ADD_SELF)
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
fn main() {
    let blockchain: SharedChain = Arc::new(Mutex::new(Blockchain::new()));
    let sockets: Arc<Mutex<Vec<SharedSocket>>> = Arc::new(Mutex::new(Vec::<SharedSocket>::new()));

    let server_bc = Arc::clone(&blockchain);
    let server_sockets = Arc::clone(&sockets);
    let http_server_handle = thread::spawn(move || {
        http_server::init_http(server_bc, server_sockets);
    });
    let blockchain_node = Arc::clone(&blockchain);
    init_node(blockchain_node, sockets);

    let blockchain = Arc::clone(&blockchain);
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_secs(1));
        let timestamp = datetime::Instant::now().seconds();
        if timestamp % 60 == 0 {
            let mut guard = blockchain.lock().unwrap();
            guard.withdraw();
            drop(guard)
        }
    });

    http_server_handle
        .join()
        .expect("Failed to terminate thread");
}
