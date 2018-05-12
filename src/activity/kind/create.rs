use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct CreateType;

impl Serialize for CreateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Create")
    }
}

pub struct CreateTypeVisitor;

impl<'de> Visitor<'de> for CreateTypeVisitor {
    type Value = CreateType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Create'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Create" {
            Ok(CreateType)
        } else {
            Err(de::Error::custom("Type not create"))
        }
    }
}

impl<'de> Deserialize<'de> for CreateType {
    fn deserialize<D>(deserializer: D) -> Result<CreateType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CreateTypeVisitor)
    }
}
