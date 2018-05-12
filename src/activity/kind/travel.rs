use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct TravelType;

impl Serialize for TravelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Travel")
    }
}

pub struct TravelTypeVisitor;

impl<'de> Visitor<'de> for TravelTypeVisitor {
    type Value = TravelType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Travel'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Travel" {
            Ok(TravelType)
        } else {
            Err(de::Error::custom("Type not Travel"))
        }
    }
}

impl<'de> Deserialize<'de> for TravelType {
    fn deserialize<D>(deserializer: D) -> Result<TravelType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TravelTypeVisitor)
    }
}
