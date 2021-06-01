use redis::{Commands, ConnectionAddr};

fn main() {
    let val = fetch_cache().unwrap();
    println!("{}", val)
}

fn fetch_cache() -> redis::RedisResult<isize> {
    let client = redis::Client::open(redis::ConnectionInfo {
        addr: Box::new(ConnectionAddr::Tcp("127.0.0.1".to_string(), 6379)),
        db: 0,
        username: None,
        passwd: Some("123456".to_string()),
    })?;
    let mut conn = client.get_connection()?;
    conn.set("test-key", 25)?;
    conn.get("test-key")
}
