use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct EventType;

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Event")
    }
}

pub struct EventTypeVisitor;

impl<'de> Visitor<'de> for EventTypeVisitor {
    type Value = EventType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Event'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Event" {
            Ok(EventType)
        } else {
            Err(de::Error::custom("Type not Event"))
        }
    }
}

impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<EventType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(EventTypeVisitor)
    }
}
