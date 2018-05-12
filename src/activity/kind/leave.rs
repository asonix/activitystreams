use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct LeaveType;

impl Serialize for LeaveType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Leave")
    }
}

pub struct LeaveTypeVisitor;

impl<'de> Visitor<'de> for LeaveTypeVisitor {
    type Value = LeaveType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Leave'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Leave" {
            Ok(LeaveType)
        } else {
            Err(de::Error::custom("Type not Leave"))
        }
    }
}

impl<'de> Deserialize<'de> for LeaveType {
    fn deserialize<D>(deserializer: D) -> Result<LeaveType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(LeaveTypeVisitor)
    }
}
