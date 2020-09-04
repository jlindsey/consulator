use std::env;

use serde_derive::{Deserialize, Serialize};

use consulator::KVValue;

const DEFAULT_CONSUL_ADDR: &str = "http://127.0.0.1:8500";

#[derive(Debug)]
struct Settings {
    consul_addr: String,
    consul_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct KV {
    key: String,
    value: KVValue,
}

impl Settings {
    fn new() -> Self {
        let consul_addr =
            env::var("CONSUL_HTTP_ADDR").unwrap_or_else(|_| DEFAULT_CONSUL_ADDR.into());
        let consul_token = env::var("CONSUL_HTTP_TOKEN").ok();
        Self {
            consul_addr,
            consul_token,
        }
    }
}

#[tokio::main]
async fn main() {
    let settings = Settings::new();
    println!("{:?}", settings);
}
