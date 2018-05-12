use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct MoveType;

impl Serialize for MoveType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Move")
    }
}

pub struct MoveTypeVisitor;

impl<'de> Visitor<'de> for MoveTypeVisitor {
    type Value = MoveType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Move'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Move" {
            Ok(MoveType)
        } else {
            Err(de::Error::custom("Type not Move"))
        }
    }
}

impl<'de> Deserialize<'de> for MoveType {
    fn deserialize<D>(deserializer: D) -> Result<MoveType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MoveTypeVisitor)
    }
}
