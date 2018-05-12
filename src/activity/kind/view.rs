use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ViewType;

impl Serialize for ViewType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("View")
    }
}

pub struct ViewTypeVisitor;

impl<'de> Visitor<'de> for ViewTypeVisitor {
    type Value = ViewType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'View'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "View" {
            Ok(ViewType)
        } else {
            Err(de::Error::custom("Type not View"))
        }
    }
}

impl<'de> Deserialize<'de> for ViewType {
    fn deserialize<D>(deserializer: D) -> Result<ViewType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ViewTypeVisitor)
    }
}
