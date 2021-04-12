use lib::rpc::rpc_protocol::RpcProtocol;
use lib::rpc::rpc_server::RpcServer;
use async_std::prelude::*;
use async_std::task;
use std::sync::mpsc;

pub struct SeamlessServer {
    rpc_clients: Vec<RpcProtocol>,
}

impl SeamelessServer {
    pub fn init(seamless) {
        let rpc_server = Rp
        task::spawn(async {
            let mut client_protocol = RpcProtocol::new(stream);
            RpcServer::connection_loop(&mut client_protocol).await;
        });cServer::init();
    }
}
