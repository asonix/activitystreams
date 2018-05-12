use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct AcceptType;

impl Serialize for AcceptType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Accept")
    }
}

pub struct AcceptTypeVisitor;

impl<'de> Visitor<'de> for AcceptTypeVisitor {
    type Value = AcceptType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Accept'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Accept" {
            Ok(AcceptType)
        } else {
            Err(de::Error::custom("Type not Accept"))
        }
    }
}

impl<'de> Deserialize<'de> for AcceptType {
    fn deserialize<D>(deserializer: D) -> Result<AcceptType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AcceptTypeVisitor)
    }
}
