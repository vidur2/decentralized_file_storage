use std::net::{TcpListener, TcpStream};
use std::sync::{ Arc, Mutex };
use std::thread;
use std::io::{Read, Write};
use crate::blockchain;
use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::block::Block;
use serde::Deserialize;
use tungstenite::{accept, WebSocket};

enum ConnectionType {
    Webocket,
    Http,
    Failuire
}

#[derive(Deserialize)]
enum MessageType {
    Chain(Vec<Block>),
    Block(Block)
}

pub fn init_http() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    let mut blockchain: Arc<Mutex<Blockchain>> = Arc::new(Mutex::new(Blockchain::new()));
    let sockets: Arc<Mutex<Vec<Arc<Mutex<WebSocket<TcpStream>>>>>> = Arc::new(Mutex::new(Vec::<Arc<Mutex<WebSocket<TcpStream>>>>::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let conn_type = check_connection_type(&mut stream.try_clone().unwrap());

        match conn_type {
            ConnectionType::Http => {
                handle_http(stream);
            },

            ConnectionType::Webocket => {
                let sockets = sockets.clone();
                let current_iter_bc = blockchain.clone();

                thread::spawn(move || {
                    let ws = Arc::new(Mutex::new(accept(stream).unwrap()));
                    let mut socket_guard = sockets.lock().unwrap();
                    socket_guard.append(&mut vec![ws.clone()]);
                    drop(socket_guard);
                    loop {
                        let msg = ws.lock().unwrap().read_message().unwrap();
            
                        match msg {
                            tungstenite::Message::Text(block_infor) => {
                                let parsed: MessageType = serde_json::from_str(&block_infor).unwrap();
                                let mut gaurded = current_iter_bc.lock().unwrap();
                                let ws_iter = sockets.lock().unwrap();
                                let reffed = serde_json::to_string_pretty(&gaurded.0).unwrap();

                                match parsed {
                                    MessageType::Chain(new_bc) => {
                                        let ran = gaurded.replace_chain(new_bc);
                                        
                                        if ran {
                                            for socket in  ws_iter.iter() {
                                                let mut socket_writable = socket.lock().unwrap();
                                                socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                                drop(socket_writable);
                                            }
                                        }
                                    }
                                    MessageType::Block(new_block) => {
                                        let ran = gaurded.add_unverified_block(new_block);

                                        if ran {
                                            for socket in  ws_iter.iter() {
                                                let mut socket_writable = socket.lock().unwrap();
                                                socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                                drop(socket_writable);
                                            }
                                        }
                                    },
                                }
                            },
                            tungstenite::Message::Binary(_) => todo!(),
                            _ => {
                                println!("Invalid ws format")
                            }
                        }
                    }
                });
            },

            ConnectionType::Failuire => {
                todo!();
            }
        }
    }
}

fn handle_listener(listener: TcpListener, blockchain: Arc<Mutex<Blockchain>>, sockets: &'static mut Arc<Mutex<Vec<Mutex<WebSocket<TcpStream>>>>>) {
    
}

fn check_connection_type(stream: &mut TcpStream) -> ConnectionType {
    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer) {
        Ok(_) => {
            let comparable = String::from_utf8(buffer.to_vec()).unwrap();
            if comparable.contains("Connection: Upgrade") {
                return ConnectionType::Webocket;
            } else {
                return ConnectionType::Http;
            }
        }

        Err(_err) => {
            return ConnectionType::Failuire;
        }
    };
}

fn handle_http(stream: TcpStream) {todo!()}
