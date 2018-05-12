use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct CollectionType;

impl Serialize for CollectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Collection")
    }
}

pub struct CollectionTypeVisitor;

impl<'de> Visitor<'de> for CollectionTypeVisitor {
    type Value = CollectionType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Collection'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Collection" {
            Ok(CollectionType)
        } else {
            Err(de::Error::custom("Type not Collection"))
        }
    }
}

impl<'de> Deserialize<'de> for CollectionType {
    fn deserialize<D>(deserializer: D) -> Result<CollectionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CollectionTypeVisitor)
    }
}
