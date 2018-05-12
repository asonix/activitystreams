use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct FlagType;

impl Serialize for FlagType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Flag")
    }
}

pub struct FlagTypeVisitor;

impl<'de> Visitor<'de> for FlagTypeVisitor {
    type Value = FlagType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Flag'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Flag" {
            Ok(FlagType)
        } else {
            Err(de::Error::custom("Type not Flag"))
        }
    }
}

impl<'de> Deserialize<'de> for FlagType {
    fn deserialize<D>(deserializer: D) -> Result<FlagType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FlagTypeVisitor)
    }
}
