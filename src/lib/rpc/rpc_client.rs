use async_std::net::TcpStream;
use crate::lib::rpc::rpc_protocol::RpcProtocol;
use crate::lib::rpc::rpc_message::RpcMessage;

use super::rpc_protocol;

struct RpcClient {
    tcp_server: TcpStream,
    protocol: RpcProtocol,
}

impl RpcClient {
    pub fn send(&mut self, msg: RpcMessage) {
        let serialized = bincode::serialize(&msg).unwrap();
        self.protocol.write(&mut self.tcp_server, &serialized)
    }

    pub async fn connection_loop () {

    }
}