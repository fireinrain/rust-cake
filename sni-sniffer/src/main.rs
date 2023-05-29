#[warn(unused_imports)]

use native_tls::TlsConnector;
use std::error::Error;
use std::net::ToSocketAddrs;
use tokio::io::{AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = "129.232.166.106:443"
        .to_socket_addrs()?
        .next()
        .ok_or("failed to resolve ip")?;

    let socket = TcpStream::connect(&addr).await?;

    let cx = TlsConnector::builder().build()?;
    let tls_cx = tokio_native_tls::TlsConnector::from(cx);

    let socket = tls_cx.connect("www.cloudflare.com", socket).await;

    match socket {
        Ok(mut stream_socket) => {
            println!("ip is a cloudflare sni proxy!");
            let shutdown = stream_socket.shutdown();
            shutdown.await.unwrap();
        }
        Err(error) => {
            println!("ip isn't a cloudflare sni proxy {:?}", error);
        }
    };


    Ok(())
}