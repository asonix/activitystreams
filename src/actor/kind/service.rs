use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ServiceType;

impl Serialize for ServiceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Service")
    }
}

pub struct ServiceTypeVisitor;

impl<'de> Visitor<'de> for ServiceTypeVisitor {
    type Value = ServiceType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Service'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Service" {
            Ok(ServiceType)
        } else {
            Err(de::Error::custom("Type not Service"))
        }
    }
}

impl<'de> Deserialize<'de> for ServiceType {
    fn deserialize<D>(deserializer: D) -> Result<ServiceType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ServiceTypeVisitor)
    }
}
