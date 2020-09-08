mod consul;
mod errors;
mod kv;
mod settings;

pub use consul::update_consul;
pub use errors::{Errors, Result};
pub use kv::KV;
pub use settings::Settings;
