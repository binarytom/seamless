use crate::lib::rpc::rpc_client_proxy::RpcClientProxy;
use async_std::io;
use async_std::net::{TcpListener, ToSocketAddrs};
use async_std::prelude::*;
use std::sync::mpsc;

pub struct RpcServer {}

impl RpcServer {
    /// accept loop for rpc_server
    ///
    /// # Examples
    /// start a accept loop
    ///
    ///``` no_run
    /// let (sender, receiver) = mpsc::channel();
    /// task::spawn(async {
    ///     RpcServer::accept_loop("127.0.0.1:10666", sender).await;
    /// });
    /// for mut client_proxy in receiver {
    ///     ....
    /// }
    ///```
    pub async fn accept_loop(
        to_socket_addrs: impl ToSocketAddrs,
        sender: mpsc::Sender<RpcClientProxy>,
    ) -> io::Result<()> {
        let listener = TcpListener::bind(to_socket_addrs).await?;

        let mut incoming = listener.incoming();
        while let Some(stream) = incoming.next().await {
            println!("receive accept of client");
            let stream = stream?;

            sender.send(RpcClientProxy::new(stream)).unwrap();
        }
        Ok(())
    }
}
