use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct IgnoreType;

impl Serialize for IgnoreType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Ignore")
    }
}

pub struct IgnoreTypeVisitor;

impl<'de> Visitor<'de> for IgnoreTypeVisitor {
    type Value = IgnoreType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Ignore'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Ignore" {
            Ok(IgnoreType)
        } else {
            Err(de::Error::custom("Type not Ignore"))
        }
    }
}

impl<'de> Deserialize<'de> for IgnoreType {
    fn deserialize<D>(deserializer: D) -> Result<IgnoreType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(IgnoreTypeVisitor)
    }
}
