use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ListenType;

impl Serialize for ListenType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Listen")
    }
}

pub struct ListenTypeVisitor;

impl<'de> Visitor<'de> for ListenTypeVisitor {
    type Value = ListenType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Listen'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Listen" {
            Ok(ListenType)
        } else {
            Err(de::Error::custom("Type not Listen"))
        }
    }
}

impl<'de> Deserialize<'de> for ListenType {
    fn deserialize<D>(deserializer: D) -> Result<ListenType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ListenTypeVisitor)
    }
}
