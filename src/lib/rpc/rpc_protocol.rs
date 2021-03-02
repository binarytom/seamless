use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;

pub struct RpcProtocol {
    size_buf: [u8; 4],
}

impl RpcProtocol {

    pub fn new() -> RpcProtocol {
        let mut size_buf: [u8; 4] = [0u8; 4];
        RpcProtocol {size_buf}
    }

    pub async fn read(&mut self, tcp_stream: &mut TcpStream) -> io::Result<(Vec<u8>)> {
        let size = self.read_size(tcp_stream).await?;
        let data = self.read_data(tcp_stream, size).await?;
        Ok((data))
    }

    async fn read_size(&mut self, tcp_stream: &mut TcpStream) -> io::Result<u32> {
        tcp_stream.read_exact(&mut self.size_buf).await?;
        let size = u32::from_be_bytes(self.size_buf);
        Ok(size)
    }

    async fn read_data(&mut self, tcp_stream: &mut TcpStream, size: u32) -> io::Result<Vec<u8>> {
        let mut ret: Vec<u8> = vec![0u8; size as usize];
        tcp_stream.read_exact(&mut ret).await?;
        Ok(ret)
    }

    pub fn write(&mut self, tcp_stream: &mut TcpStream, data: &Vec<u8>) -> () {
        let size = data.len() as u32;
        tcp_stream.write(size.to_be_bytes().as_ref());
        tcp_stream.write(data);
        tcp_stream.flush();
    }
}
