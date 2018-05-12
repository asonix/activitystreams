use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct PlaceType;

impl Serialize for PlaceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Place")
    }
}

pub struct PlaceTypeVisitor;

impl<'de> Visitor<'de> for PlaceTypeVisitor {
    type Value = PlaceType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Place'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Place" {
            Ok(PlaceType)
        } else {
            Err(de::Error::custom("Type not Place"))
        }
    }
}

impl<'de> Deserialize<'de> for PlaceType {
    fn deserialize<D>(deserializer: D) -> Result<PlaceType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PlaceTypeVisitor)
    }
}
