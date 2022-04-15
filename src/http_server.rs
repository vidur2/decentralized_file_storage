use crate::blockchain::block::Block;
use crate::blockchain::blockchain::SharedChain;
use crate::blockchain::file_infor::BlockInformation;
use serde::Deserialize;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{accept, WebSocket};

pub type SharedSocket = Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>;

#[derive(Deserialize)]
enum MessageType {
    Chain(Vec<Block>),
    Block(Block),
}

pub fn init_http(blockchain: SharedChain, sockets: Arc<Mutex<Vec<SharedSocket>>>) {
    let http_listener = TcpListener::bind("0.0.0.0:8002").unwrap();
    let ws_listener = TcpListener::bind("0.0.0.0:8003").unwrap();

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
                let ws = Arc::new(Mutex::new(accept(MaybeTlsStream::Plain(stream)).unwrap()));
                let mut socket_guard = sockets.lock().unwrap();
                socket_guard.append(&mut vec![Arc::clone(&ws)]);
                drop(socket_guard);

                // Handle user off
                let ws_ctrlc = Arc::clone(&ws);
                ctrlc::set_handler(move || {
                    let mut ws_guard = ws_ctrlc.lock().unwrap();
                    ws_guard.close(None).expect("Could not close");
                })
                .expect("Could not add listener");

                handle_socket_connection(ws, blockchain, sockets)
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
        thread::spawn(move || handle_http(&mut stream, blockchain, sockets));
    }
}

// Handles the two diff types of socket connections the same way
pub fn handle_socket_connection(
    ws: SharedSocket,
    blockchain: SharedChain,
    sockets: Arc<Mutex<Vec<SharedSocket>>>,
) {
    _handle_ws_connection(ws, blockchain, sockets)
}

/// Handles ws connection message, mainly changing blockchain/verfication
///
/// ## Arguments
/// * `ws_uw`: The socket being handled
/// * `blockchain`: The data on the blockchain
/// * `sockets`: A vector of the shared websockets
fn _handle_ws_connection(
    ws_uw: SharedSocket,
    blockchain: SharedChain,
    sockets: Arc<Mutex<Vec<SharedSocket>>>,
) {
    loop {
        let mut read_guarded = ws_uw.lock().unwrap();
        let msg = read_guarded.read_message().unwrap();
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
                            for socket in ws_iter.iter() {
                                let mut socket_writable = socket.lock().unwrap();
                                socket_writable
                                    .write_message(tungstenite::Message::Text(reffed.clone()))
                                    .expect("Could not send blockchain message");
                            }
                        }
                    }
                    MessageType::Block(new_block) => {
                        let ran = guarded.add_unverified_block(new_block);

                        if ran {
                            for socket in ws_iter.iter() {
                                let mut socket_writable = socket.lock().unwrap();
                                socket_writable
                                    .write_message(tungstenite::Message::Text(reffed.clone()))
                                    .expect("Could not send blockchain message");
                            }
                        }
                    }
                }
            }
            tungstenite::Message::Binary(_) => todo!(),
            _ => {
                println!("Invalid ws format")
            }
        }
    }
}

fn handle_http(
    stream: &mut TcpStream,
    blockchain: SharedChain,
    sockets: Arc<Mutex<Vec<SharedSocket>>>,
) {
    // Initialization of reading var
    let mut buffer = [0u8; 1024];
    let mut response_content = String::new();

    // Handles stream
    match stream.read(&mut buffer) {
        Ok(_) => {
            // Route to store a file on chain
            // Takes a FileInformation struct as input
            // `data` field should be a base64 url with mime type if it is frontend, otherwise it can be stored as any format, you just have to handle it
            if buffer.starts_with(b"POST /store_information HTTP/1.1") {
                let full_req = String::from_utf8(buffer.to_vec()).unwrap();
                let body = parse_body(full_req);
                let file_infor_in_body: BlockInformation = serde_json::from_str(&body).unwrap();
                let mut guard = blockchain.lock().unwrap();
                guard.add_block(file_infor_in_body);

                let ws_iter = sockets.lock().unwrap();
                let reffed = serde_json::to_string_pretty(&guard.0).unwrap();

                for socket in ws_iter.iter() {
                    let mut socket_writable = socket.lock().unwrap();
                    socket_writable
                        .write_message(tungstenite::Message::Text(reffed.clone()))
                        .expect("Could not send blockchain message");
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
                let guarded = blockchain.lock().unwrap();
                let block = guarded.find_block_by_uri(&body);
                drop(body);

                match block {
                    Some(block_uw) => {
                        let data = &block_uw.data.data;
                        // Write response here
                        let response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                            data.as_ref().unwrap().len(),
                            data.as_ref().unwrap()
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
                    }
                }
            }
            // Return blocks from blockchain
            else if buffer.starts_with(b"GET /get_blocks HTTP/1.1") {
                let blockchain = blockchain.lock().unwrap();
                let blocks_as_str = serde_json::to_string(&blockchain.0).unwrap();
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    blocks_as_str.len(),
                    blocks_as_str
                );

                response_content.push_str(&response);
            }
            // Returns hash given timestamp
            else if buffer.starts_with(b"POST /get_hash HTTP/1.1") {
                let timestamp: i128 = parse_body(String::from_utf8(buffer.to_vec()).unwrap())
                    .trim()
                    .parse()
                    .unwrap();
                let blockchain = blockchain.lock().unwrap();
                let mut hash: &str = "No matching hash";

                for block in blockchain.0.iter().rev() {
                    if block.data.timestamp == timestamp {
                        hash = &block.hash
                    }
                }

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    hash.len(),
                    hash
                );

                response_content.push_str(&response);
            }

            stream.write(response_content.as_bytes()).unwrap();
        }
        Err(_) => todo!(),
    }
}

fn parse_body(body: String) -> String {
    let split_string: Vec<&str> = body.split("Content-Length: ").collect();
    let content_len = split_string[1];
    let content_len_split: Vec<&str> = content_len.split("\n").collect();
    let content_len_int: usize = content_len_split[0]
        .trim()
        .parse()
        .expect("Could not cast to integer");
    let split_body: Vec<&str> = body.split("\n").collect();

    String::from(split_body[split_body.len() - 1])
        .split_at(content_len_int)
        .0
        .to_string()
}
