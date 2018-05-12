use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct TombstoneType;

impl Serialize for TombstoneType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Tombstone")
    }
}

pub struct TombstoneTypeVisitor;

impl<'de> Visitor<'de> for TombstoneTypeVisitor {
    type Value = TombstoneType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Tombstone'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Tombstone" {
            Ok(TombstoneType)
        } else {
            Err(de::Error::custom("Type not Tombstone"))
        }
    }
}

impl<'de> Deserialize<'de> for TombstoneType {
    fn deserialize<D>(deserializer: D) -> Result<TombstoneType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TombstoneTypeVisitor)
    }
}
