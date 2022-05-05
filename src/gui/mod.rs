pub mod gui;
mod node_assist;
mod request_type;

const MIDDLEWARE_ADDR_GET_BLOCKS: &str = "http://localhost:8080/get_blocks";
const MIDDLEWARE_ADDR_GET_PEERS: &str = "http://localhost:8080/get_peers";
const MIDDLEWARE_ADDR_ADD_SELF: &str = "http://localhost:8080/add_self_as_peer";
const MIDDLEWARE_ADDR_REMOVE_SELF: &str = "http://localhost:8080/remove_ip_addr";
