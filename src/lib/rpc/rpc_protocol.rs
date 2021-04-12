use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use std::sync::Arc;

/// rpc protocol is a protocol based on tcp
/// provide read and write from net.
pub struct RpcProtocol {
    size_buf: [u8; 4],
    tcp_stream_arc: Arc<TcpStream>,
}

impl RpcProtocol {
    pub fn new(tcp_stream: TcpStream) -> RpcProtocol {
        let size_buf: [u8; 4] = [0u8; 4];
        let tcp_stream_arc = Arc::new(tcp_stream);
        RpcProtocol {
            size_buf,
            tcp_stream_arc,
        }
    }

    pub async fn read(&mut self) -> io::Result<Vec<u8>> {
        let size = self.read_size().await?;
        let data = self.read_data(size).await?;
        Ok(data)
    }

    async fn read_size(&mut self) -> io::Result<u32> {
        let mut tcp_stream = &*self.tcp_stream_arc;
        tcp_stream.read_exact(&mut self.size_buf).await?;
        let size = u32::from_be_bytes(self.size_buf);
        Ok(size)
    }

    async fn read_data(&mut self, size: u32) -> io::Result<Vec<u8>> {
        let mut tcp_stream = &*self.tcp_stream_arc;
        let mut ret: Vec<u8> = vec![0u8; size as usize];
        tcp_stream.read_exact(&mut ret).await?;
        Ok(ret)
    }

    pub async fn write(&mut self, data: &Vec<u8>) -> io::Result<()> {
        let mut tcp_stream = &*self.tcp_stream_arc;
        let size = data.len() as u32;
        tcp_stream.write(size.to_be_bytes().as_ref()).await?;
        tcp_stream.write(data).await?;
        tcp_stream.flush().await?;
        Ok(())
    }
}
