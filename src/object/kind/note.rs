use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct NoteType;

impl Serialize for NoteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Note")
    }
}

pub struct NoteTypeVisitor;

impl<'de> Visitor<'de> for NoteTypeVisitor {
    type Value = NoteType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Note'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Note" {
            Ok(NoteType)
        } else {
            Err(de::Error::custom("Type not Note"))
        }
    }
}

impl<'de> Deserialize<'de> for NoteType {
    fn deserialize<D>(deserializer: D) -> Result<NoteType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(NoteTypeVisitor)
    }
}
