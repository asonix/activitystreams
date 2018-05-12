use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct JoinType;

impl Serialize for JoinType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Join")
    }
}

pub struct JoinTypeVisitor;

impl<'de> Visitor<'de> for JoinTypeVisitor {
    type Value = JoinType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Join'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Join" {
            Ok(JoinType)
        } else {
            Err(de::Error::custom("Type not Join"))
        }
    }
}

impl<'de> Deserialize<'de> for JoinType {
    fn deserialize<D>(deserializer: D) -> Result<JoinType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(JoinTypeVisitor)
    }
}
