use std::{env, fmt, ops::Deref};

const DEFAULT_CONSUL_ADDR: &str = "http://127.0.0.1:8500";

/// Simple wrapper around a type to keep it from being printed. The wrapped value
/// can still be printed if you wish by accessing it directly with .0
pub struct Secret<T>(pub T);
impl<T> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret{...}")
    }
}
impl<T> fmt::Display for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("****")
    }
}

impl<T> Deref for Secret<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Settings {
    pub consul_addr: String,
    pub consul_token: Secret<Option<String>>,
}

impl Settings {
    pub fn new() -> Self {
        let consul_addr =
            env::var("CONSUL_HTTP_ADDR").unwrap_or_else(|_| DEFAULT_CONSUL_ADDR.into());
        let consul_token = Secret(env::var("CONSUL_HTTP_TOKEN").ok());
        Self {
            consul_addr,
            consul_token,
        }
    }
}
