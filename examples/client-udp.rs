use std::{net::SocketAddr, str::FromStr};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use udp_stream::UdpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = UdpStream::connect(SocketAddr::from_str("127.0.0.1:8080").unwrap()).await?;
    println!("Ready to Connected to {}", &stream.peer_addr()?);
    loop {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        stream.write_all(buffer.as_bytes()).await?;
        let mut buf = vec![0u8; 1024];
        let n = stream.read(&mut buf).await?;
        print!("-> {}", String::from_utf8_lossy(&buf[..n]));
    }
}
