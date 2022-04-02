mod blockchain;
mod http_server;

const MIDDLEWARE_ADDR: &str = "http://localhost:8080";

fn init_node() {

}

fn main() {
    http_server::init_http();
}
