use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct AudioType;

impl Serialize for AudioType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Audio")
    }
}

pub struct AudioTypeVisitor;

impl<'de> Visitor<'de> for AudioTypeVisitor {
    type Value = AudioType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Audio'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Audio" {
            Ok(AudioType)
        } else {
            Err(de::Error::custom("Type not Audio"))
        }
    }
}

impl<'de> Deserialize<'de> for AudioType {
    fn deserialize<D>(deserializer: D) -> Result<AudioType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AudioTypeVisitor)
    }
}
