use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct OrganizationType;

impl Serialize for OrganizationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Organization")
    }
}

pub struct OrganizationTypeVisitor;

impl<'de> Visitor<'de> for OrganizationTypeVisitor {
    type Value = OrganizationType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Organization'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Organization" {
            Ok(OrganizationType)
        } else {
            Err(de::Error::custom("Type not Organization"))
        }
    }
}

impl<'de> Deserialize<'de> for OrganizationType {
    fn deserialize<D>(deserializer: D) -> Result<OrganizationType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OrganizationTypeVisitor)
    }
}
