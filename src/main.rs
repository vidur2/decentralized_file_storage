#![feature(let_chains)]

use std::{
    sync::{Arc, Mutex},
    thread,
};

use blockchain::blockchain::{Blockchain, SharedChain};
use http_server::SharedSocket;

mod blockchain;
mod gui;
mod http_server;
mod tests;
mod util;

fn main() {
    let blockchain: SharedChain = Arc::new(Mutex::new(Blockchain::new()));
    let sockets: Arc<Mutex<Vec<SharedSocket>>> = Arc::new(Mutex::new(Vec::<SharedSocket>::new()));

    let server_bc = Arc::clone(&blockchain);
    let server_sockets = Arc::clone(&sockets);
    let initialized = Arc::new(Mutex::new(util::shared_bool::SharedBool::new(false)));
    thread::spawn(move || {
        http_server::init_http(server_bc, server_sockets);
    });
    let blockchain_node = Arc::clone(&blockchain);

    let blockchain = Arc::clone(&blockchain);
    let initialized_comp = Arc::clone(&initialized);
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_secs(1));
        let timestamp = datetime::Instant::now().seconds();
        if timestamp % 60 == 0 && initialized_comp.lock().unwrap().get_value() != false {
            let mut guard = blockchain.lock().unwrap();
            guard.withdraw();
            drop(guard)
        }
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Login",
        options,
        Box::new(|_cc| {
            Box::new(crate::gui::gui::GuiImpl::new(
                blockchain_node,
                sockets,
                initialized,
            ))
        }),
    );
    // init_node(blockchain_node, sockets
}
