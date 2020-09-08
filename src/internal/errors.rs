use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("input must be a valid JSON object, at: {0}")]
    MustBeObject(String),
    #[error("nulls and arrays not supported, at: {0}")]
    UnsupportedType(String),
}
