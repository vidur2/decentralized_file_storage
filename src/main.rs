use std::{
    sync::{Arc, Mutex},
    thread,
};

use blockchain::{
    block::Block,
    blockchain::{Blockchain, SharedChain},
};
use http_server::SharedSocket;
use serde::Serialize;

use crate::http_server::handle_socket_connection;

mod blockchain;
mod http_server;

const MIDDLEWARE_ADDR_GET_BLOCKS: &str = "http://localhost:8080/get_blocks";
const MIDDLEWARE_ADDR_GET: &str = "http://localhost:8080/get_peers";
const MIDDLEWARE_ADDR_POST: &str = "http://localhost:8080/add_self_as_peer";

#[derive(Serialize)]
struct IpInformation {
    socket_addr: String,
    http_addr: String,
}

fn get_addr() -> IpInformation {
    let mut socket_addr = String::new();
    let mut http_addr = String::new();

    println!("Enter the address for your node's socket endpoint (ws://)): ");
    std::io::stdin().read_line(&mut socket_addr).unwrap();

    println!("Enter the address of your node's http endpoint (http://): ");
    std::io::stdin().read_line(&mut http_addr).unwrap();

    let split_socket_addr: Vec<&str> = socket_addr.split("\n").collect();
    let split_http_addr: Vec<&str> = http_addr.split("\n").collect();

    return IpInformation {
        socket_addr: String::from(split_socket_addr[0]),
        http_addr: String::from(split_http_addr[0]),
    };
}

fn init_node(
    blockchain: crate::blockchain::blockchain::SharedChain,
    sockets: Arc<Mutex<Vec<SharedSocket>>>,
) {
    let client = reqwest::blocking::Client::new();

    let addr = get_addr();
    let req_body_infor = serde_json::to_string(&addr).unwrap();

    let resp = client
        .post(MIDDLEWARE_ADDR_POST)
        .body(req_body_infor)
        .send()
        .unwrap()
        .text()
        .unwrap();

    if &resp == "true" {
        drop(resp);
        let resp = client
            .post(MIDDLEWARE_ADDR_GET)
            .body(addr.socket_addr)
            .send()
            .unwrap()
            .text()
            .unwrap();

        let mut reffed_bc = blockchain.lock().unwrap();

        if resp != "" {
            let blockchain_str = reqwest::blocking::get(MIDDLEWARE_ADDR_GET_BLOCKS)
                .unwrap()
                .text()
                .unwrap();

            let blockchain_vec: Vec<Block> = serde_json::from_str(&blockchain_str).unwrap();
            drop(blockchain_str);

            *reffed_bc = Blockchain(blockchain_vec);

            drop(reffed_bc);

            let parsed_response: Vec<&str> = resp.split(",").collect();

            for host in parsed_response.iter() {
                let blockchain = Arc::clone(&blockchain);
                let ws = Arc::new(Mutex::new(tungstenite::client::connect(*host).unwrap().0));
                let sockets = Arc::clone(&sockets);
                sockets.lock().unwrap().push(Arc::clone(&ws));

                thread::spawn(move || {
                    handle_socket_connection(ws, blockchain, sockets);
                });
            }
        } else {
            *reffed_bc = Blockchain::new();
        }
    }
    println!("Done!");
}
fn main() {
    let blockchain: SharedChain = Arc::new(Mutex::new(Blockchain::new()));
    let sockets: Arc<Mutex<Vec<SharedSocket>>> = Arc::new(Mutex::new(Vec::<SharedSocket>::new()));

    let server_bc = Arc::clone(&blockchain);
    let server_sockets = Arc::clone(&sockets);
    let http_server_handle = thread::spawn(move || {
        http_server::init_http(server_bc, server_sockets);
    });

    // Comment in  when go server has been tested
    init_node(blockchain, sockets);

    http_server_handle
        .join()
        .expect("Failed to terminate thread");
}
