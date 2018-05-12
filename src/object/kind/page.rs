use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct PageType;

impl Serialize for PageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Page")
    }
}

pub struct PageTypeVisitor;

impl<'de> Visitor<'de> for PageTypeVisitor {
    type Value = PageType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Page'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Page" {
            Ok(PageType)
        } else {
            Err(de::Error::custom("Type not Page"))
        }
    }
}

impl<'de> Deserialize<'de> for PageType {
    fn deserialize<D>(deserializer: D) -> Result<PageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PageTypeVisitor)
    }
}
