use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct RemoveType;

impl Serialize for RemoveType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Remove")
    }
}

pub struct RemoveTypeVisitor;

impl<'de> Visitor<'de> for RemoveTypeVisitor {
    type Value = RemoveType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Remove'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Remove" {
            Ok(RemoveType)
        } else {
            Err(de::Error::custom("Type not Remove"))
        }
    }
}

impl<'de> Deserialize<'de> for RemoveType {
    fn deserialize<D>(deserializer: D) -> Result<RemoveType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RemoveTypeVisitor)
    }
}
