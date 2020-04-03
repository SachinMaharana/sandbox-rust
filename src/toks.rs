use tokio::net::TcpStream;
use tokio::prelude::*;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let mut stream = TcpStream::connect(addr).await.unwrap();
    println!("Created Stream");

    let result = stream.write(b"Sachin,Tokio").await;
    println!("Wrote to stream; success={:?}", result.is_ok());
}
