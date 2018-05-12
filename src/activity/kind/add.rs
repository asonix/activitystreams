use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct AddType;

impl Serialize for AddType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Add")
    }
}

pub struct AddTypeVisitor;

impl<'de> Visitor<'de> for AddTypeVisitor {
    type Value = AddType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Add'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Add" {
            Ok(AddType)
        } else {
            Err(de::Error::custom("Type not create"))
        }
    }
}

impl<'de> Deserialize<'de> for AddType {
    fn deserialize<D>(deserializer: D) -> Result<AddType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AddTypeVisitor)
    }
}
