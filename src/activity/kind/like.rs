use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct LikeType;

impl Serialize for LikeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Like")
    }
}

pub struct LikeTypeVisitor;

impl<'de> Visitor<'de> for LikeTypeVisitor {
    type Value = LikeType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Like'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Like" {
            Ok(LikeType)
        } else {
            Err(de::Error::custom("Type not Like"))
        }
    }
}

impl<'de> Deserialize<'de> for LikeType {
    fn deserialize<D>(deserializer: D) -> Result<LikeType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LikeTypeVisitor)
    }
}
