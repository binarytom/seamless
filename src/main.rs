mod lib;
use async_std::io;
use async_std::net::TcpStream;
use async_std::task;
use bincode;
use io::Result;
use lib::rpc::rpc_message::RpcMessage;
use lib::rpc::rpc_server::RpcServer;

fn main() -> Result<()> {
    // task::block_on(RpcServer::accept_loop())
    //let content = "Hello World!".to_string();
    //let rpc_message = RpcMessage::ContentMessage { content };
    //let bytes = bincode::serialize(&rpc_message).unwrap();
    //println!("{:?}", bytes);
    //let deserized = bincode::deserialize::<RpcMessage>(&bytes).unwrap();
    //println!("{:?}", deserized);
    //Ok(())
    //println!("Hello, world!");
    //task::block_on(async {
    //    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8080").await?;
    //    // let rpc_protocol = RpcProtocol(stream);
    //    let msg = "Hello World!";
    //    let mut buf = vec![0u8; 1024];
    //    let n = stream.peek(&mut buf).await?;
    Ok(())
    //})
}
