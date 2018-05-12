use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct TentativeRejectType;

impl Serialize for TentativeRejectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("TentativeReject")
    }
}

pub struct TentativeRejectTypeVisitor;

impl<'de> Visitor<'de> for TentativeRejectTypeVisitor {
    type Value = TentativeRejectType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'TentativeReject'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "TentativeReject" {
            Ok(TentativeRejectType)
        } else {
            Err(de::Error::custom("Type not TentativeReject"))
        }
    }
}

impl<'de> Deserialize<'de> for TentativeRejectType {
    fn deserialize<D>(deserializer: D) -> Result<TentativeRejectType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TentativeRejectTypeVisitor)
    }
}
