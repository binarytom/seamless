use crate::lib::rpc::rpc_server_proxy::RpcServerProxy;
use async_std::io::Result;
use async_std::net::{TcpStream, ToSocketAddrs};

pub struct RpcClient {}

impl RpcClient {
    /// connect to a server
    /// Examples
    ///
    /// connect to server
    ///
    /// ```no_run
    /// let mut server_proxy = RpcClient::connect("127.0.0.1:10666").await;
    /// server_proxy.connection_loop().await;
    /// ```
    ///
    pub async fn connect(addr: impl ToSocketAddrs) -> Result<RpcServerProxy> {
        println!("connect to server!");
        let stream = TcpStream::connect(addr).await?;
        let rpc_server_proxy = RpcServerProxy::new(stream);
        Ok(rpc_server_proxy)
    }
}
