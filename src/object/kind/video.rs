use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct VideoType;

impl Serialize for VideoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Video")
    }
}

pub struct VideoTypeVisitor;

impl<'de> Visitor<'de> for VideoTypeVisitor {
    type Value = VideoType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Video'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Video" {
            Ok(VideoType)
        } else {
            Err(de::Error::custom("Type not Video"))
        }
    }
}

impl<'de> Deserialize<'de> for VideoType {
    fn deserialize<D>(deserializer: D) -> Result<VideoType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(VideoTypeVisitor)
    }
}
