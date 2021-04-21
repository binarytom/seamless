#![feature(min_type_alias_impl_trait)]
#![feature(impl_trait_in_bindings)]
mod lib;
use async_std::io;
use async_std::task;
use io::Result;
use std::sync::mpsc;
use std::{thread, time};

fn main() -> Result<()> {
    test_window();
    Ok(())
}

fn test_window() {
    use lib::platform::window::Window;
    use lib::platform::x_windows_monitor::XWindows;

    let x_win = XWindows::new();
    x_win.x_hide_cursor();
}

fn test_rpc() {
    use lib::rpc::rpc_client::RpcClient;
    use lib::rpc::rpc_client_proxy::RpcClientProxy;
    use lib::rpc::rpc_message::RpcMessage;
    use lib::rpc::rpc_server::RpcServer;
    // task::block_on(RpcServer::accept_loop())
    // let content = "Hello World!".to_string();
    // let rpc_message = RpcMessage::ContentMessage { content };
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
    // Ok(())
    //})
    let (sender, receiver) = mpsc::channel();
    task::spawn(async {
        RpcServer::accept_loop("127.0.0.1:10666", sender)
            .await
            .unwrap();
    });
    thread::spawn(move || {
        let mut i = 0;
        loop {
            i = i + 1;
            let mut client_proxy: RpcClientProxy = receiver.recv().unwrap();
            thread::spawn(move || {
                // send hello
                loop {
                    thread::sleep(time::Duration::from_millis(i * 2000));
                    println!("send hello msg to client {:?}", i);
                    let content = format!("Hello World! client {:?}", i);
                    // let content = "Hello World!".to_string();
                    let rpc_message = RpcMessage::ContentMessage { content };
                    task::block_on(async {
                        client_proxy.send(rpc_message).await.unwrap();
                    });
                }
            });
        }
        // for mut client_proxy in receiver {
        // }
    });

    let ten_millis = time::Duration::from_millis(1000);
    thread::sleep(ten_millis);
    task::spawn(async {
        let mut server_proxy = RpcClient::connect("127.0.0.1:10666").await.unwrap();
        server_proxy.connection_loop().await;
    });

    task::spawn(async {
        let mut server_proxy = RpcClient::connect("127.0.0.1:10666").await.unwrap();
        server_proxy.connection_loop().await;
    });
    loop {}
}
