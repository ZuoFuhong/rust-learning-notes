use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

fn main() {
    untyped_example().unwrap();
    typed_example().unwrap();
}

fn untyped_example() -> Result<()> {
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
    let v: Value = serde_json::from_str(data)?;
    println!(
        "name = {}, first phone number = {}",
        v["name"], v["phones"][0]
    );
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
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
    let p: Person = serde_json::from_str(data)?;
    println!("name = {}, first phone number = {}", p.name, p.phones[0]);
    let s = serde_json::to_string(&p)?;
    println!("{}", s);
    Ok(())
}
