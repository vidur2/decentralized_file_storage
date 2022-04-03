use std::{sync::{Mutex, Arc}, thread, net::TcpStream};

use blockchain::blockchain::{SharedChain, Blockchain};
use http_server::{SharedSocket, WsOption};
use tungstenite::WebSocket;

use crate::http_server::handle_socket_connection;

mod blockchain;
mod http_server;

const MIDDLEWARE_ADDR_GET: &str = "http://localhost:8080/get_peers";
const MIDDLEWARE_ADDR_POST: &str = "http://localhost:8080/add_self_as_peer";

fn init_node(blockchain: crate::blockchain::blockchain::SharedChain, sockets: Arc<Mutex<Vec<WsOption>>>) {

    use local_ip_address::local_ip;

    let resp = reqwest::blocking::get(MIDDLEWARE_ADDR_GET)
                                    .unwrap()
                                    .text()
                                    .unwrap();
    let parsed_response: Vec<&str> = serde_json::from_str(&resp).unwrap();

    for host in parsed_response.iter() {

        let blockchain = Arc::clone(&blockchain);
        let ws = Arc::new(Mutex::new(tungstenite::client::connect(*host).unwrap().0));
        let sockets = Arc::clone(&sockets);
        sockets.lock().unwrap().push(WsOption::Client(Arc::clone(&ws)));

        thread::spawn(move || {
            handle_socket_connection(WsOption::Client(ws), blockchain, sockets);
        });
    }

    let current_ip = serde_json::to_string(&local_ip().unwrap()).unwrap();
    let client = reqwest::blocking::Client::new();

    let resp = client.post(MIDDLEWARE_ADDR_POST)
                                .body(current_ip)
                                .send()
                                .unwrap();
}
fn main() {
    let blockchain: SharedChain = Arc::new(Mutex::new(Blockchain::new()));
    let sockets: Arc<Mutex<Vec<WsOption>>> = Arc::new(Mutex::new(Vec::<WsOption>::new()));
    init_node(Arc::clone(&blockchain), Arc::clone(&sockets));
    http_server::init_http(blockchain, sockets);
}
