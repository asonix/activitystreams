use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct CollectionPageType;

impl Serialize for CollectionPageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("CollectionPage")
    }
}

pub struct CollectionPageTypeVisitor;

impl<'de> Visitor<'de> for CollectionPageTypeVisitor {
    type Value = CollectionPageType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'CollectionPage'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "CollectionPage" {
            Ok(CollectionPageType)
        } else {
            Err(de::Error::custom("Type not CollectionPage"))
        }
    }
}

impl<'de> Deserialize<'de> for CollectionPageType {
    fn deserialize<D>(deserializer: D) -> Result<CollectionPageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CollectionPageTypeVisitor)
    }
}
