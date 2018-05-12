use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct OrderedCollectionType;

impl Serialize for OrderedCollectionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("OrderedCollection")
    }
}

pub struct OrderedCollectionTypeVisitor;

impl<'de> Visitor<'de> for OrderedCollectionTypeVisitor {
    type Value = OrderedCollectionType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'OrderedCollection'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "OrderedCollection" {
            Ok(OrderedCollectionType)
        } else {
            Err(de::Error::custom("Type not OrderedCollection"))
        }
    }
}

impl<'de> Deserialize<'de> for OrderedCollectionType {
    fn deserialize<D>(deserializer: D) -> Result<OrderedCollectionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OrderedCollectionTypeVisitor)
    }
}
