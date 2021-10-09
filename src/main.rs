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
    net::TcpListener,
};

#[tokio::main]
pub async fn main() -> Result<(), io::Error> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:6142".to_string());
    let listener = TcpListener::bind(&addr).await?;
    let hm: HashMap<String, String> = HashMap::new();
    let store = Store(Arc::new(RwLock::new(hm)));

    loop {
        let this_store = Store(store.0.clone());

        // Asynchronously wait for an inbound TcpStream.
        let (mut stream, _) = listener.accept().await?;
        let mut buf: Vec<u8> = Vec::new();
        stream.read_to_end(&mut buf).await?;
        let mut cmd: String = String::from_utf8(buf).unwrap();
        let mut cmd = cmd.split(" ");
        if let Some(base_cmd) = cmd.nth(0) {
            if let Some(primary_arg) = cmd.nth(0) {
                if let Some(sec_arg) = cmd.nth(0) {}
            }
        }
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
    async fn get<I>(&self, pattern: K) -> Result<V, String>
    where
        I: Iterator<Item = V>,
    {
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
