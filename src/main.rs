use std::{
    collections::HashMap,
    env,
    fmt::Display,
    hash::Hash,
    io,
    sync::{Arc, PoisonError, RwLock},
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
fn read_till_char(stream: &TcpStream, c: u8) -> String {}
#[tokio::main]
pub async fn main() -> Result<(), io::Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());
    let listener = TcpListener::bind(&addr).await?;
    let hm: HashMap<String, String> = HashMap::new();
    let store = Store(Arc::new(RwLock::new(hm)));

    loop {
        let mut this_store = Store(store.0.clone());

        // Asynchronously wait for an inbound TcpStream.
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            stream.read(&mut buf).await.unwrap();
            let mut cmd: String = String::from_utf8_lossy(&buf[..]).to_string();
            println!("Request => {}", cmd);
            let mut cmd = cmd.split(" ");
            let base_cmd = cmd.nth(0).unwrap();

            let primary_arg = cmd.nth(0).unwrap();
            if base_cmd == "set" {
                let value_arg = cmd.nth(0).unwrap();
                this_store
                    .set(primary_arg.to_string(), value_arg.to_string())
                    .await
                    .unwrap();
                stream.write(b"SUCCESS").await;
            } else if base_cmd == "get" {
                let value = this_store.get(primary_arg.to_string()).await.unwrap();
                stream.write(value.as_bytes()).await;
            }
        });
    }
}
struct Store<K, V>(Arc<RwLock<HashMap<K, V>>>)
where
    K: Eq + Hash + Display,
    V: Clone;

impl<K, V> Store<K, V>
where
    K: Eq + Hash + Display,
    V: Clone,
{
    async fn set(
        &mut self,
        key: K,
        value: V,
    ) -> Result<(), PoisonError<std::sync::RwLockWriteGuard<'_, HashMap<K, V>>>> {
        let mut s = self.0.write()?;
        s.insert(key, value);
        Ok(())
    }
    async fn get(&self, pattern: K) -> Result<V, String> {
        let s = self.0.read();
        match s {
            Ok(s) => {
                if let Some(val) = s.get(&pattern) {
                    Ok(val.clone())
                } else {
                    Err(format!("cannot find the given pattern {}", pattern).to_string())
                }
            }
            Err(err) => return Err("".to_string()),
        }
    }
}
