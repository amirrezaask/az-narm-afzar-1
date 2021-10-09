use std::{env, io};

use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), io::Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());
    let listener = TcpListener::bind(&addr).await?;

    loop {
        // Asynchronously wait for an inbound TcpStream.
        let (stream, addr) = listener.accept().await?;
    }
}
