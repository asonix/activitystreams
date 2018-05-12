use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct UndoType;

impl Serialize for UndoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Undo")
    }
}

pub struct UndoTypeVisitor;

impl<'de> Visitor<'de> for UndoTypeVisitor {
    type Value = UndoType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Undo'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Undo" {
            Ok(UndoType)
        } else {
            Err(de::Error::custom("Type not Undo"))
        }
    }
}

impl<'de> Deserialize<'de> for UndoType {
    fn deserialize<D>(deserializer: D) -> Result<UndoType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(UndoTypeVisitor)
    }
}
