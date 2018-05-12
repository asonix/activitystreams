use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct OrderedCollectionPageType;

impl Serialize for OrderedCollectionPageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("OrderedCollectionPage")
    }
}

pub struct OrderedCollectionPageTypeVisitor;

impl<'de> Visitor<'de> for OrderedCollectionPageTypeVisitor {
    type Value = OrderedCollectionPageType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'OrderedCollectionPage'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "OrderedCollectionPage" {
            Ok(OrderedCollectionPageType)
        } else {
            Err(de::Error::custom("Type not OrderedCollectionPage"))
        }
    }
}

impl<'de> Deserialize<'de> for OrderedCollectionPageType {
    fn deserialize<D>(deserializer: D) -> Result<OrderedCollectionPageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OrderedCollectionPageTypeVisitor)
    }
}
