use std::net::{TcpListener, TcpStream};
use std::sync::{ Arc, Mutex };
use std::thread;
use std::io::{Read, Write};
use crate::blockchain::blockchain::Blockchain;
use crate::blockchain::block::Block;
use crate::blockchain::file_infor::FileInformation;
use serde::Deserialize;
use tungstenite::{accept, WebSocket};

type SharedChain =  Arc<Mutex<Blockchain>>;
type SharedSocket = Arc<Mutex<WebSocket<TcpStream>>>;

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
    let blockchain: SharedChain = Arc::new(Mutex::new(Blockchain::new()));
    let sockets: Arc<Mutex<Vec<SharedSocket>>> = Arc::new(Mutex::new(Vec::<SharedSocket>::new()));

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let conn_type = check_connection_type(&mut stream.try_clone().unwrap());
        let sockets = sockets.clone();
        let blockchain = blockchain.clone();

        match conn_type {
            ConnectionType::Http => {
                handle_http(&mut stream, blockchain, sockets);
            },

            ConnectionType::Webocket => {

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
                                let mut guarded = blockchain.lock().unwrap();
                                let ws_iter = sockets.lock().unwrap();
                                let reffed = serde_json::to_string_pretty(&guarded.0).unwrap();

                                match parsed {
                                    MessageType::Chain(new_bc) => {
                                        let ran = guarded.replace_chain(new_bc);
                                        
                                        if ran {
                                            for socket in  ws_iter.iter() {
                                                let mut socket_writable = socket.lock().unwrap();
                                                socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                                drop(socket_writable);
                                            }
                                        }
                                    }
                                    MessageType::Block(new_block) => {
                                        let ran = guarded.add_unverified_block(new_block);

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

fn handle_http(stream: &mut TcpStream, blockchain: SharedChain, sockets: Arc<Mutex<Vec<SharedSocket>>>) {
    let mut buffer = [0u8; 1024];
    let response_content = Arc::new(Mutex::new(String::new()));
    match stream.read(&mut buffer) {
        Ok(_) => {
            let cloned_response_ref = response_content.clone();
            thread::spawn(move || {
                if buffer.starts_with(b"POST /store_information") {
                    let full_req = String::from_utf8(buffer.to_vec()).unwrap();
                    let body = parse_body(full_req);
                    let file_infor_in_body: FileInformation = serde_json::from_str(&body).unwrap();
                    let mut guard = blockchain.lock().unwrap();
                    guard.add_block(file_infor_in_body);

                    let ws_iter = sockets.lock().unwrap();
                    let reffed = serde_json::to_string_pretty(&guard.0).unwrap();

                    for socket in  ws_iter.iter() {
                        let mut socket_writable = socket.lock().unwrap();
                        socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                    }
                } else if buffer.starts_with(b"POST /get_information_by_url") {
                    let full_req = String::from_utf8(buffer.to_vec()).unwrap();
                    let body = parse_body(full_req);
                    let guarded=  blockchain.lock().unwrap();
                    let block = guarded.find_block_by_uri(&body);
                    drop(body);
                    
                    match block {
                        Some(block_uw) => {
                            let data = &block_uw.data.data;
                            // Write response here 
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                                data.len(),
                                &data
                            );
                            cloned_response_ref.lock().unwrap().push_str(&response);
                            drop(response);
                        }
                        None => {
                            let resp = "URL not stored in blockchain";
                            let response = format!(
                                "HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}",
                                resp.len(),
                                resp
                            );
                            cloned_response_ref.lock().unwrap().push_str(&response);
                            drop(response);
                        },
                    }
                }
            });
            stream.write(response_content.lock().unwrap().as_bytes()).unwrap();
        },
        Err(_) => todo!(),
    }
}

fn parse_body(body: String) -> String {
    let split_string: Vec<&str> = body.split("Content-Length: ").collect();
    let content_len = split_string[1];
    let content_len_split: Vec<&str> = content_len.split("\n").collect();
    let content_len_int: usize = content_len_split[0].trim().parse().expect("Could not cast to integer");
    let split_body: Vec<&str> = body.split("\n").collect();
    println!("{}", split_body[split_body.len() - 1]);
    String::from(split_body[split_body.len() - 1]).split_at(content_len_int).0.to_string()
}
