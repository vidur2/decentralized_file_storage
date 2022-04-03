use std::{sync::{Mutex, Arc}, thread};

use blockchain::{blockchain::{SharedChain, Blockchain}};
use http_server::WsOption;

use crate::http_server::handle_socket_connection;

mod blockchain;
mod http_server;

const MIDDLEWARE_ADDR_GET: &str = "http://localhost:8080/get_peers";
const MIDDLEWARE_ADDR_POST: &str = "http://localhost:8080/add_self_as_peer";

fn init_node(blockchain: crate::blockchain::blockchain::SharedChain, sockets: Arc<Mutex<Vec<WsOption>>>) {

    let resp = reqwest::blocking::get(MIDDLEWARE_ADDR_POST)
                                    .unwrap()
                                    .text()
                                    .unwrap();

    if resp == "true" {
        let mut reffed_bc = blockchain.lock().unwrap();

        *reffed_bc = Blockchain::new();

        drop(reffed_bc);

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
    }
}
fn main() {
    let blockchain: SharedChain = Arc::new(Mutex::new(Blockchain::new()));
    let sockets: Arc<Mutex<Vec<WsOption>>> = Arc::new(Mutex::new(Vec::<WsOption>::new()));
    http_server::init_http(Arc::clone(&blockchain), Arc::clone(&sockets));
    /*
    // Comment in  when go server has been tested
    init_node(blockchain, sockets);
    */
}
