use std::io::{self, Read};

mod internal;

pub use internal::Result;
use internal::{update_consul, Errors, Settings, KV};

use serde_json::Value;

pub async fn run() -> Result<()> {
    let settings = Settings::new();
    println!("{:?}", settings);

    let mut buf = Vec::new();
    let mut stdin = io::stdin();
    stdin.read_to_end(&mut buf)?;

    let parsed: Value = serde_json::from_slice(&buf)?;
    if let Some(obj) = parsed.as_object() {
        let kvs = walk_json(obj, "")?;
        println!("{:?}", kvs);
        println!("{}", serde_json::to_string_pretty(&kvs)?);
        update_consul(&settings, kvs).await
    } else {
        Err(Errors::MustBeObject("/".to_string()).into())
    }
}

fn walk_json(obj: &serde_json::Map<String, Value>, key_path: &str) -> Result<Vec<KV>> {
    let mut out: Vec<KV> = Vec::new();

    for (k, v) in obj {
        let mut key = String::from(key_path);
        key.push_str(format!("/{}", k).as_str());

        match v {
            Value::Number(num) => out.push(KV::new(key, num.to_string())),
            Value::String(s) => out.push(KV::new(key, s.clone())),
            Value::Bool(b) => out.push(KV::new(key, b.to_string())),
            Value::Object(obj) => out.append(&mut walk_json(obj, &key)?),
            _ => return Err(Errors::UnsupportedType(key).into()),
        }
    }
    Ok(out)
}
