use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct RejectType;

impl Serialize for RejectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Reject")
    }
}

pub struct RejectTypeVisitor;

impl<'de> Visitor<'de> for RejectTypeVisitor {
    type Value = RejectType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Reject'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Reject" {
            Ok(RejectType)
        } else {
            Err(de::Error::custom("Type not Reject"))
        }
    }
}

impl<'de> Deserialize<'de> for RejectType {
    fn deserialize<D>(deserializer: D) -> Result<RejectType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RejectTypeVisitor)
    }
}
