use std::{env, io};

use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::main]
pub async fn main() -> Result<(), io::Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());
    let listener = TcpListener::bind(&addr).await?;

    loop {
        // Asynchronously wait for an inbound TcpStream.
        let (mut stream, _) = listener.accept().await?;
        match stream.write(b"Amirreza says hello").await {
            Ok(_) => {}
            Err(err) => println!("err in writing: {}", err),
        }
    }
}
