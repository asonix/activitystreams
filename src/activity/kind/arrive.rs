use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ArriveType;

impl Serialize for ArriveType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Arrive")
    }
}

pub struct ArriveTypeVisitor;

impl<'de> Visitor<'de> for ArriveTypeVisitor {
    type Value = ArriveType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Arrive'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Arrive" {
            Ok(ArriveType)
        } else {
            Err(de::Error::custom("Type not Arrive"))
        }
    }
}

impl<'de> Deserialize<'de> for ArriveType {
    fn deserialize<D>(deserializer: D) -> Result<ArriveType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ArriveTypeVisitor)
    }
}
