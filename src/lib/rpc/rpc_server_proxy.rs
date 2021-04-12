use crate::lib::rpc::rpc_message::RpcMessage;
use crate::lib::rpc::rpc_protocol::RpcProtocol;
use async_std::io::Result;
use async_std::net::TcpStream;

pub struct RpcServerProxy {
    server_protocol: RpcProtocol,
}

impl RpcServerProxy {
    pub fn new(stream: TcpStream) -> RpcServerProxy {
        let server_protocol = RpcProtocol::new(stream);
        RpcServerProxy { server_protocol }
    }

    pub async fn send(&mut self, msg: RpcMessage) -> Result<()> {
        let serialized = bincode::serialize(&msg).unwrap();
        self.server_protocol.write(&serialized).await?;
        Ok(())
    }

    pub async fn connection_loop(&mut self) {
        println!("start connection loop");
        loop {
            let ret = self.server_protocol.read().await;
            match ret {
                Ok(msg_bytes) => RpcServerProxy::handle_msg(msg_bytes),
                Err(error) => println!("Problem opening the file: {:?}", error),
            }
        }
    }

    fn handle_msg(bytes: Vec<u8>) {
        let deserialized = bincode::deserialize::<RpcMessage>(&bytes).unwrap();
        println!("recv msg: {:?}", deserialized);
    }
}
