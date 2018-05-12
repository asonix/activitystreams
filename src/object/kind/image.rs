use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ImageType;

impl Serialize for ImageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Image")
    }
}

pub struct ImageTypeVisitor;

impl<'de> Visitor<'de> for ImageTypeVisitor {
    type Value = ImageType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Image'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Image" {
            Ok(ImageType)
        } else {
            Err(de::Error::custom("Type not Image"))
        }
    }
}

impl<'de> Deserialize<'de> for ImageType {
    fn deserialize<D>(deserializer: D) -> Result<ImageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ImageTypeVisitor)
    }
}
