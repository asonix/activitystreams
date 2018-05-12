use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct FollowType;

impl Serialize for FollowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Follow")
    }
}

pub struct FollowTypeVisitor;

impl<'de> Visitor<'de> for FollowTypeVisitor {
    type Value = FollowType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Follow'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Follow" {
            Ok(FollowType)
        } else {
            Err(de::Error::custom("Type not Follow"))
        }
    }
}

impl<'de> Deserialize<'de> for FollowType {
    fn deserialize<D>(deserializer: D) -> Result<FollowType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FollowTypeVisitor)
    }
}
