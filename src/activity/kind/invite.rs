use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct InviteType;

impl Serialize for InviteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Invite")
    }
}

pub struct InviteTypeVisitor;

impl<'de> Visitor<'de> for InviteTypeVisitor {
    type Value = InviteType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Invite'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Invite" {
            Ok(InviteType)
        } else {
            Err(de::Error::custom("Type not Invite"))
        }
    }
}

impl<'de> Deserialize<'de> for InviteType {
    fn deserialize<D>(deserializer: D) -> Result<InviteType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(InviteTypeVisitor)
    }
}
