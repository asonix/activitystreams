use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct AnnounceType;

impl Serialize for AnnounceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Announce")
    }
}

pub struct AnnounceTypeVisitor;

impl<'de> Visitor<'de> for AnnounceTypeVisitor {
    type Value = AnnounceType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Announce'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Announce" {
            Ok(AnnounceType)
        } else {
            Err(de::Error::custom("Type not Announce"))
        }
    }
}

impl<'de> Deserialize<'de> for AnnounceType {
    fn deserialize<D>(deserializer: D) -> Result<AnnounceType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AnnounceTypeVisitor)
    }
}
