use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ReadType;

impl Serialize for ReadType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Read")
    }
}

pub struct ReadTypeVisitor;

impl<'de> Visitor<'de> for ReadTypeVisitor {
    type Value = ReadType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Read'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Read" {
            Ok(ReadType)
        } else {
            Err(de::Error::custom("Type not Read"))
        }
    }
}

impl<'de> Deserialize<'de> for ReadType {
    fn deserialize<D>(deserializer: D) -> Result<ReadType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ReadTypeVisitor)
    }
}
