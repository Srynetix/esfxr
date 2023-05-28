use std::ops::{Deref, DerefMut};

use fundsp::shared::Shared;
use serde::{de::Visitor, Deserialize, Serialize, Serializer};

#[derive(Clone)]
pub struct SerializableSharedF64(Shared<f64>);

impl SerializableSharedF64 {
    pub fn new(value: f64) -> Self {
        Self(Shared::new(value))
    }
}

impl From<SerializableSharedF64> for Shared<f64> {
    fn from(value: SerializableSharedF64) -> Self {
        value.0
    }
}

impl Deref for SerializableSharedF64 {
    type Target = Shared<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SerializableSharedF64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Serialize for SerializableSharedF64 {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.0.value())
    }
}

struct SerializableSharedF64Visitor;

impl<'de> Visitor<'de> for SerializableSharedF64Visitor {
    type Value = SerializableSharedF64;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a float value")
    }

    fn visit_f32<E>(self, v: f32) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Self::Value::new(v as f64))
    }

    fn visit_f64<E>(self, v: f64) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Self::Value::new(v))
    }
}

impl<'de> Deserialize<'de> for SerializableSharedF64 {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_f64(SerializableSharedF64Visitor)
    }
}
