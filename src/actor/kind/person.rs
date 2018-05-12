use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct PersonType;

impl Serialize for PersonType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Person")
    }
}

pub struct PersonTypeVisitor;

impl<'de> Visitor<'de> for PersonTypeVisitor {
    type Value = PersonType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Person'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Person" {
            Ok(PersonType)
        } else {
            Err(de::Error::custom("Type not Person"))
        }
    }
}

impl<'de> Deserialize<'de> for PersonType {
    fn deserialize<D>(deserializer: D) -> Result<PersonType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PersonTypeVisitor)
    }
}
