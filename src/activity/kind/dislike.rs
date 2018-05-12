use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct DislikeType;

impl Serialize for DislikeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Dislike")
    }
}

pub struct DislikeTypeVisitor;

impl<'de> Visitor<'de> for DislikeTypeVisitor {
    type Value = DislikeType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Dislike'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Dislike" {
            Ok(DislikeType)
        } else {
            Err(de::Error::custom("Type not Dislike"))
        }
    }
}

impl<'de> Deserialize<'de> for DislikeType {
    fn deserialize<D>(deserializer: D) -> Result<DislikeType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DislikeTypeVisitor)
    }
}
