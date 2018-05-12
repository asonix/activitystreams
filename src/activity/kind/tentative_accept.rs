use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct TentativeAcceptType;

impl Serialize for TentativeAcceptType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("TentativeAccept")
    }
}

pub struct TentativeAcceptTypeVisitor;

impl<'de> Visitor<'de> for TentativeAcceptTypeVisitor {
    type Value = TentativeAcceptType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'TentativeAccept'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "TentativeAccept" {
            Ok(TentativeAcceptType)
        } else {
            Err(de::Error::custom("Type not TentativeAccept"))
        }
    }
}

impl<'de> Deserialize<'de> for TentativeAcceptType {
    fn deserialize<D>(deserializer: D) -> Result<TentativeAcceptType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TentativeAcceptTypeVisitor)
    }
}
