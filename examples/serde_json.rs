use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Value};
use std::collections::HashMap;

fn main() {
    untyped_example();
    typed_example();
    hashmap_example();
    stream_into_iter();
}

fn untyped_example() {
    let data = r#"
        {
            "name": "mars",
            "age": 24,
            "phones": [
                "18870000000",
                "18880000000"
            ]
        }    
    "#;
    let v: Value = serde_json::from_str(data).unwrap();
    println!(
        "name = {}, first phone number = {}",
        v["name"], v["phones"][0]
    );
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() {
    let data = r#"
        {
            "name": "mars",
            "age": 24,
            "phones": [
                "18870000000",
                "18880000000"
            ]
        }
    "#;
    let p: Person = serde_json::from_str(data).unwrap();
    println!("name = {}, first phone number = {}", p.name, p.phones[0]);
    let s = serde_json::to_string(&p).unwrap();
    println!("{}", s);
}

#[derive(Deserialize, Debug)]
struct Data {
    foo: String,
}

fn hashmap_example() {
    let data = r#"{
        "a": {"foo": "bar"},
        "b": {"foo": "baz"}
    }"#;
    let m: HashMap<String, Data> = serde_json::from_str(data).unwrap();
    for (k, v) in m {
        println!("k = {}, v = {:?}", k, v)
    }
}

// 将流反序列化为多个JSON值的迭代器
fn stream_into_iter() {
    let data = "{\"k\": 3}1\"cool\"\"stuff\" 3{}  [0, 1, 2]";
    let stream = Deserializer::from_str(data).into_iter::<Value>();
    for value in stream {
        println!("{}", value.unwrap())
    }
}
