use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct RelationshipType;

impl Serialize for RelationshipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Relationship")
    }
}

pub struct RelationshipTypeVisitor;

impl<'de> Visitor<'de> for RelationshipTypeVisitor {
    type Value = RelationshipType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Relationship'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Relationship" {
            Ok(RelationshipType)
        } else {
            Err(de::Error::custom("Type not Relationship"))
        }
    }
}

impl<'de> Deserialize<'de> for RelationshipType {
    fn deserialize<D>(deserializer: D) -> Result<RelationshipType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RelationshipTypeVisitor)
    }
}
