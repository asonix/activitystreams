use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct GroupType;

impl Serialize for GroupType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Group")
    }
}

pub struct GroupTypeVisitor;

impl<'de> Visitor<'de> for GroupTypeVisitor {
    type Value = GroupType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Group'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Group" {
            Ok(GroupType)
        } else {
            Err(de::Error::custom("Type not Group"))
        }
    }
}

impl<'de> Deserialize<'de> for GroupType {
    fn deserialize<D>(deserializer: D) -> Result<GroupType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(GroupTypeVisitor)
    }
}
