use crate::lib::rpc::rpc_message::RpcMessage;
use crate::lib::rpc::rpc_protocol::RpcProtocol;
use async_std::net::TcpStream;

struct RpcClient {
    tcp_server: TcpStream,
    protocol: RpcProtocol,
}

impl RpcClient {
    pub async fn init() -> RpcClient {
        let mut tcp_server = TcpStream::connect("127.0.0.1:6666").await.unwrap();
        let mut protocol = RpcProtocol::new();
        let mut rpc_client = RpcClient {
            tcp_server,
            protocol,
        };

        rpc_client.connection_loop();

        rpc_client
    }

    pub fn send(&mut self, msg: RpcMessage) {
        let serialized = bincode::serialize(&msg).unwrap();
        self.protocol.write(&mut self.tcp_server, &serialized);
    }

    pub async fn connection_loop(&mut self) {
        loop {
            let ret = self.protocol.read(&mut self.tcp_server).await.unwrap();
            let deserialized = bincode::deserialize::<RpcMessage>(&ret).unwrap();
            println!("recv msg: {:?}", deserialized);
        }
    }
}
