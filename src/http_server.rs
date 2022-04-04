use std::net::{TcpListener, TcpStream};
use std::sync::{ Arc, Mutex };
use std::thread;
use std::io::{Read, Write};
use crate::blockchain::blockchain::{SharedChain};
use crate::blockchain::block::Block;
use crate::blockchain::file_infor::FileInformation;
use serde::Deserialize;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{accept, WebSocket};


pub type SharedSocket = Arc<Mutex<WebSocket<TcpStream>>>;
pub type SharedSocketSafe = Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

pub enum WsOption {
    Server(SharedSocket),
    Client(SharedSocketSafe)
}


#[derive(Deserialize)]
enum MessageType {
    Chain(Vec<Block>),
    Block(Block)
}

pub fn init_http(blockchain: SharedChain, sockets: Arc<Mutex<Vec<WsOption>>>) {
    let http_listener = TcpListener::bind("127.0.0.1:8002").unwrap();
    let ws_listener = TcpListener::bind("127.0.0.1:8003").unwrap();

    // Create pointers for thread
    let ws_blockchain = Arc::clone(&blockchain);
    let ws_sockets = Arc::clone(&sockets);
    thread::spawn(move || {

        // General handler for ws conn
        for stream in ws_listener.incoming() {
            let stream = stream.unwrap();
            let blockchain = Arc::clone(&ws_blockchain);
            let sockets = Arc::clone(&ws_sockets);

            thread::spawn(move || {

                // Bringing pointers into thread
                let ws = Arc::new(Mutex::new(accept(stream).unwrap()));
                let mut socket_guard = sockets.lock().unwrap();
                socket_guard.append(&mut vec![WsOption::Server(Arc::clone(&ws))]);
                drop(socket_guard);

                // Handle user off
                let ws_ctrlc = Arc::clone(&ws);
                ctrlc::set_handler(move || {
                    let mut ws_guard = ws_ctrlc.lock().unwrap();
                    ws_guard.close(None).expect("Could not close");
                }).expect("Could not add listener");


                handle_socket_connection(WsOption::Server(ws), blockchain, sockets)
            });
        }
    });

    // Create pointers for http server
    let http_blockchain = Arc::clone(&blockchain);
    let http_sockets = Arc::clone(&sockets);

    // Http Handler
    for stream in http_listener.incoming() {
        let mut stream = stream.unwrap();
        let blockchain = Arc::clone(&http_blockchain);
        let sockets = Arc::clone(&http_sockets);
        thread::spawn(move || {
            handle_http(&mut stream, blockchain, sockets)
        });
    }
}

// Handles the two diff types of socket connections the same way
pub fn handle_socket_connection(ws: WsOption, blockchain: SharedChain, sockets: Arc<Mutex<Vec<WsOption>>>) {

    match ws {
        WsOption::Client(ws_uw) => {
            _handle_ws_connection_client(ws_uw, blockchain, sockets)
        }
        
        WsOption::Server(ws_uw) => {
            _handle_ws_connection_server(ws_uw, blockchain, sockets)
        },
    }
}

fn _handle_ws_connection_client(ws_uw: SharedSocketSafe, blockchain: SharedChain, sockets: Arc<Mutex<Vec<WsOption>>>) {
    loop {
        let msg = ws_uw.lock().unwrap().read_message().unwrap();
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

                                if let WsOption::Server(socket_uw) = socket {
                                    let mut socket_writable = socket_uw.lock().unwrap();
                                    socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                } else if let WsOption::Client(socket_uw)= socket {
                                    let mut socket_writable = socket_uw.lock().unwrap();
                                    socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                }
                            }
                        }
                    }
                    MessageType::Block(new_block) => {
                        let ran = guarded.add_unverified_block(new_block);

                        if ran {
                            for socket in  ws_iter.iter() {

                                if let WsOption::Server(socket_uw) = socket {
                                    let mut socket_writable = socket_uw.lock().unwrap();
                                    socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                } else if let WsOption::Client(socket_uw) = socket {
                                    let mut socket_writable = socket_uw.lock().unwrap();
                                    socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                }
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
}

fn _handle_ws_connection_server(ws_uw: SharedSocket, blockchain: SharedChain, sockets: Arc<Mutex<Vec<WsOption>>>) {
    loop {
        let msg = ws_uw.lock().unwrap().read_message().unwrap();
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

                                if let WsOption::Server(socket_uw) = socket {
                                    let mut socket_writable = socket_uw.lock().unwrap();
                                    socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                } else if let WsOption::Client(socket_uw)= socket {
                                    let mut socket_writable = socket_uw.lock().unwrap();
                                    socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                }
                            }
                        }
                    }
                    MessageType::Block(new_block) => {
                        let ran = guarded.add_unverified_block(new_block);

                        if ran {
                            for socket in  ws_iter.iter() {

                                if let WsOption::Server(socket_uw) = socket {
                                    let mut socket_writable = socket_uw.lock().unwrap();
                                    socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                } else if let WsOption::Client(socket_uw) = socket {
                                    let mut socket_writable = socket_uw.lock().unwrap();
                                    socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                                }
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
}

// Deprecated
// fn check_connection_type(stream: &mut TcpStream) -> (ConnectionType, Option<String>){
//     let mut buffer = [0u8; 1024];
//     match stream.read(&mut buffer) {
//         Ok(_) => {
//             let comparable = String::from_utf8(buffer.to_vec()).unwrap();
//             if comparable.contains("Connection: Upgrade") {
//                 let split_buffer: Vec<&str> = comparable.split("\n").collect();
//                 let mut host = String::new();
//                 for line in split_buffer {
//                     if line.starts_with("Origin: ") {
//                         let owned_line = String::from(line);
//                         let split_line: Vec<&str> = owned_line.split(": ").collect();
//                         let final_parse = split_line[1].replace("\r\n", "");
//                         host.push_str(&final_parse);
//                     }
//                 }
//                 return (ConnectionType::Webocket, Some(host));
//             } else {
//                 return (ConnectionType::Http, None);
//             }
//         }

//         Err(_err) => {
//             return( ConnectionType::Failuire, None);
//         }
//     };
// }

fn handle_http(stream: &mut TcpStream, blockchain: SharedChain, sockets: Arc<Mutex<Vec<WsOption>>>) {
    let mut buffer = [0u8; 1024];
    let mut response_content = String::new();
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Route to store a file on chain
            // Takes a FileInformation struct as input
            // `data` field should be a base64 url with mime type if it is frontend, otherwise it can be stored as any format, you just have to handle it
            if buffer.starts_with(b"POST /store_information HTTP/1.1") {
                let full_req = String::from_utf8(buffer.to_vec()).unwrap();
                let body = parse_body(full_req);
                let file_infor_in_body: FileInformation = serde_json::from_str(&body).unwrap();
                let mut guard = blockchain.lock().unwrap();
                guard.add_block(file_infor_in_body);

                let ws_iter = sockets.lock().unwrap();
                let reffed = serde_json::to_string_pretty(&guard.0).unwrap();

                for socket in  ws_iter.iter() {

                    if let WsOption::Server(socket_uw) = socket {
                        let mut socket_writable = socket_uw.lock().unwrap();
                        socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                    } else if let WsOption::Client(socket_uw) = socket {
                        let mut socket_writable = socket_uw.lock().unwrap();
                        socket_writable.write_message(tungstenite::Message::Text(reffed.clone())).expect("Could not send blockchain message");
                    }
                }
                let resp = "Successful";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    resp.len(),
                    &resp
                );

                response_content.push_str(&response)

            } 
            
            // Gets data from a specified url
            else if buffer.starts_with(b"POST /get_information_by_url HTTP/1.1") {
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
                        response_content.push_str(&response);
                        drop(response);
                    }
                    None => {
                        let resp = "URL not stored in blockchain";
                        let response = format!(
                            "HTTP/1.1 400 Bad Request\r\nContent-Length: {}\r\n\r\n{}",
                            resp.len(),
                            resp
                        );
                        response_content.push_str(&response);
                        drop(response);
                    },
                }
            }
            stream.write(response_content.as_bytes()).unwrap();
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
