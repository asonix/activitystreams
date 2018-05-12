use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct UpdateType;

impl Serialize for UpdateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Update")
    }
}

pub struct UpdateTypeVisitor;

impl<'de> Visitor<'de> for UpdateTypeVisitor {
    type Value = UpdateType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Update'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Update" {
            Ok(UpdateType)
        } else {
            Err(de::Error::custom("Type not update"))
        }
    }
}

impl<'de> Deserialize<'de> for UpdateType {
    fn deserialize<D>(deserializer: D) -> Result<UpdateType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UpdateTypeVisitor)
    }
}
