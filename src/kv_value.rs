use std::{fmt, ops::Deref};

use serde::{
    de::{self, Visitor},
    ser::Serializer,
};

struct KVVisitor;

impl<'de> Visitor<'de> for KVVisitor {
    type Value = KVValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a base64-encoded string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let decoded = base64::decode(v).map_err(de::Error::custom)?;
        let s = String::from_utf8(decoded).map_err(de::Error::custom)?;
        Ok(KVValue(s))
    }
}

/// Newtype wrapper to automatically handle encoding/decoding base64 from the KV API
#[derive(Debug)]
pub struct KVValue(String);

impl Deref for KVValue {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl serde::Serialize for KVValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let encoded = base64::encode(self.as_bytes());
        serializer.serialize_str(&encoded)
    }
}

impl<'de> serde::Deserialize<'de> for KVValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_string(KVVisitor)
    }
}
