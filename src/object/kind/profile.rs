use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ProfileType;

impl Serialize for ProfileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Profile")
    }
}

pub struct ProfileTypeVisitor;

impl<'de> Visitor<'de> for ProfileTypeVisitor {
    type Value = ProfileType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Profile'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Profile" {
            Ok(ProfileType)
        } else {
            Err(de::Error::custom("Type not Profile"))
        }
    }
}

impl<'de> Deserialize<'de> for ProfileType {
    fn deserialize<D>(deserializer: D) -> Result<ProfileType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ProfileTypeVisitor)
    }
}
