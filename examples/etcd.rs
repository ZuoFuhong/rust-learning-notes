use etcd_rs::{Client, ClientConfig, DeleteRequest, KeyRange, PutRequest, RangeRequest, Result};

/// ectd终端命令
/// etcdctl set name mars
/// etcdctl get name
///
/// run example
/// cargo run --example ectd
#[tokio::main]
async fn main() -> Result<()> {
    let endpoints = vec!["http://127.0.0.1:2379".to_string()];
    let client = Client::connect(ClientConfig {
        endpoints,
        auth: None,
        tls: None,
    })
    .await?;
    println!("Put and get a key value pairs");

    let key = "name";
    let value = "mars";
    let resp = client.kv().put(PutRequest::new(key, value)).await?;
    println!("Put Response: {:?}", resp);

    let req = RangeRequest::new(KeyRange::key(key));
    let resp = client.kv().range(req).await?;
    println!("Range Response: {:?}", resp);

    let resp = client
        .kv()
        .delete(DeleteRequest::new(KeyRange::key(key)))
        .await?;
    println!("Delete Response: {:?}", resp);
    Ok(())
}
