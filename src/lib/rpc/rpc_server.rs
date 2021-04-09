use crate::lib::rpc::rpc_message::RpcMessage;
use crate::lib::rpc::rpc_protocol::RpcProtocol;
use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

pub struct RpcServer {
    tcp_clients: Vec<RpcProtocol>,
}

impl RpcServer {
    pub async fn init() -> RpcServer {
        let tcp_clients = vec![];
        let rpc_server = RpcServer { tcp_clients };
        rpc_server
    }

    /// accept loop for rpc_server
    ///
    ///
    pub async fn accept_loop(rpc_server: &mut RpcServer) -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:6666").await?;

        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            task::spawn(async {
                let mut client_protocol = RpcProtocol::new(stream);
                RpcServer::connection_loop(&mut client_protocol).await;
            });
        }
        Ok(())
    }

    pub fn send(&mut self, msg: RpcMessage) {
        let serialized = bincode::serialize(&msg).unwrap();
        // self.protocol.write(&serialized);
    }

    pub async fn connection_loop(rpc_protocol: &mut RpcProtocol) {
        loop {
            let ret = rpc_protocol.read().await.unwrap();
            let deserialized = bincode::deserialize::<RpcMessage>(&ret).unwrap();
            println!("recv msg: {:?}", deserialized);
        }
    }
}
