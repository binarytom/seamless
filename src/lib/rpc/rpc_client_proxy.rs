use crate::lib::rpc::rpc_message::RpcMessage;
use crate::lib::rpc::rpc_protocol::RpcProtocol;
use async_std::io::Result;
use async_std::net::TcpStream;

pub struct RpcClientProxy {
    client_protocol: RpcProtocol,
}

unsafe impl Send for RpcClientProxy {}

unsafe impl Sync for RpcClientProxy {}

impl RpcClientProxy {
    pub fn new(stream: TcpStream) -> RpcClientProxy {
        let client_protocol = RpcProtocol::new(stream);

        RpcClientProxy { client_protocol }
    }

    /// connection loop of client
    ///
    pub async fn connection_loop(rpc_protocol: &mut RpcProtocol) -> Result<()> {
        loop {
            let ret = rpc_protocol.read().await?;
            RpcClientProxy::handle_msg(ret);
        }
    }

    pub fn handle_msg(bytes: Vec<u8>) {
        let deserialized = bincode::deserialize::<RpcMessage>(&bytes).unwrap();
        println!("recv msg: {:?}", deserialized);
    }

    pub async fn send(&mut self, msg: RpcMessage) -> Result<()> {
        let serialized = bincode::serialize(&msg).unwrap();
        self.client_protocol.write(&serialized).await?;
        Ok(())
    }
}
