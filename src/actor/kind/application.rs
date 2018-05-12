use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ApplicationType;

impl Serialize for ApplicationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Application")
    }
}

pub struct ApplicationTypeVisitor;

impl<'de> Visitor<'de> for ApplicationTypeVisitor {
    type Value = ApplicationType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Application'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Application" {
            Ok(ApplicationType)
        } else {
            Err(de::Error::custom("Type not Application"))
        }
    }
}

impl<'de> Deserialize<'de> for ApplicationType {
    fn deserialize<D>(deserializer: D) -> Result<ApplicationType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ApplicationTypeVisitor)
    }
}
