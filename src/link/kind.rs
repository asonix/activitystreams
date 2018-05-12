use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct MentionType;

impl Serialize for MentionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Mention")
    }
}

pub struct MentionTypeVisitor;

impl<'de> Visitor<'de> for MentionTypeVisitor {
    type Value = MentionType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Mention'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Mention" {
            Ok(MentionType)
        } else {
            Err(de::Error::custom("Type not Mention"))
        }
    }
}

impl<'de> Deserialize<'de> for MentionType {
    fn deserialize<D>(deserializer: D) -> Result<MentionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MentionTypeVisitor)
    }
}
