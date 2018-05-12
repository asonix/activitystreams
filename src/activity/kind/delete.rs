use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct DeleteType;

impl Serialize for DeleteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Delete")
    }
}

pub struct DeleteTypeVisitor;

impl<'de> Visitor<'de> for DeleteTypeVisitor {
    type Value = DeleteType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Delete'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Delete" {
            Ok(DeleteType)
        } else {
            Err(de::Error::custom("Type not Delete"))
        }
    }
}

impl<'de> Deserialize<'de> for DeleteType {
    fn deserialize<D>(deserializer: D) -> Result<DeleteType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DeleteTypeVisitor)
    }
}
