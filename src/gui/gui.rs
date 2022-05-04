use std::sync::{Arc, Mutex};

use crate::blockchain::pool_infor::PoolInfor;

use crate::{
    blockchain::blockchain::SharedChain, http_server::SharedSocket, util::shared_bool::SharedBool,
};

use super::node_assist::init_node;

#[repr(u8)]
enum CurrentPage {
    Login,
    SignUp,
}

pub(crate) struct GuiImpl {
    private_key: String,
    public_key: String,
    blockchain: SharedChain,
    sockets: Arc<Mutex<Vec<SharedSocket>>>,
    state: Arc<Mutex<SharedBool>>,
    page: CurrentPage,
}

impl GuiImpl {
    pub fn new(
        blockchain: SharedChain,
        sockets: Arc<Mutex<Vec<SharedSocket>>>,
        state: Arc<Mutex<SharedBool>>,
    ) -> Self {
        Self {
            private_key: String::new(),
            public_key: String::new(),
            blockchain,
            sockets,
            state,
            page: CurrentPage::Login,
        }
    }
}

impl eframe::App for GuiImpl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            if let CurrentPage::Login = self.page {
                ui.horizontal(|horiz_ui| {
                    horiz_ui.label("Enter Public Key Here: ");
                    horiz_ui.text_edit_singleline(&mut self.public_key)
                });

                ui.horizontal(|horiz_ui| {
                    if horiz_ui.button("Login").clicked() {
                        println!("{}", self.public_key);
                        let mut guard = self.state.lock().unwrap();
                        *guard = SharedBool::new(true);
                        init_node(
                            Arc::clone(&self.blockchain),
                            Arc::clone(&self.sockets),
                            self.public_key.clone(),
                        );
                    } else if horiz_ui.button("Sign Up").clicked() {
                        let mut csprng = rand::prelude::ThreadRng::default();
                        let account = ed25519_dalek::Keypair::generate(&mut csprng);
                        let mut guard = self.state.lock().unwrap();
                        self.page = CurrentPage::SignUp;
                        *guard = SharedBool::new(true);
                        self.private_key =
                            serde_json::to_string(&account.secret.to_bytes()).unwrap();
                        self.public_key =
                            serde_json::to_string(&account.public.to_bytes()).unwrap();
                        init_node(
                            Arc::clone(&self.blockchain),
                            Arc::clone(&self.sockets),
                            self.public_key.clone(),
                        );
                    }
                });
            } else if let CurrentPage::SignUp = self.page {
                ui.label(&format!("Your public key is {}", self.public_key));
                ui.label(&format!("Your private key is {}", self.private_key));
            } else {
                let amt_nodes: Option<usize> = PoolInfor::get_amount_of_nodes();
                match amt_nodes {
                    Some(value) => {
                        ui.label(&format!("Num of peers connected {}", &value));
                    }
                    None => {
                        ui.label("Not connected");
                    }
                }
            }
        });
    }
}
