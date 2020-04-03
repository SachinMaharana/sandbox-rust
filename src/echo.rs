use futures::stream::StreamExt;
use tokio::net::TcpListener;
// use tokio::prelude::*;
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(conn) = incoming.next().await {
            match conn {
                Ok(mut sock) => {
                    tokio::spawn(async move {
                        let (mut reader, mut writer) = sock.split();

                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                println!("{} bytes", amt);
                            }
                            Err(e) => {
                                eprintln!("IO error {:?}", e);
                            }
                        }
                    });
                    println!("Accepted Connection");
                }
                Err(e) => {
                    eprintln!("Accept err= {:?}", e);
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    server.await;
}
