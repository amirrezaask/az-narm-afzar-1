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

async fn read_till_char<T: AsyncReadExt + std::marker::Unpin>(stream: &mut T, c: char) -> String {
    let mut buf = [0; 1];
    let mut output = String::new();
    loop {
        stream.read(&mut buf).await.unwrap();
        let buf_char = char::from(buf[0]);
        if buf_char == c {
            break;
        }
        output.push(buf_char)
    }
    output
}

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
            loop {
                println!("waiting for a new cmd...");
                let cmd = read_till_char(&mut stream, '\n').await;
                let mut cmd = cmd.split(" ");
                let base_cmd = cmd.nth(0).unwrap();
                let primary_arg = cmd.nth(0).unwrap();
                println!("base part: '{}' first argument:'{}'", base_cmd, primary_arg);
                if base_cmd == "set" {
                    let value_arg = cmd.nth(0).unwrap();
                    println!("{}", value_arg);
                    this_store
                        .set(primary_arg.to_string(), value_arg.to_string())
                        .await
                        .unwrap();
                    stream.write(b"SUCCESS\n").await.unwrap();
                    continue;
                } else if base_cmd == "get" {
                    println!("get handler");
                    let value = this_store
                        .get(primary_arg.to_string().trim_end().to_string())
                        .await
                        .unwrap();
                    println!("got value {}", value);
                    stream.write(value.as_bytes()).await.unwrap();
                    continue;
                }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_till_char() {}
}
