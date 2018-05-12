use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct BlockType;

impl Serialize for BlockType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Block")
    }
}

pub struct BlockTypeVisitor;

impl<'de> Visitor<'de> for BlockTypeVisitor {
    type Value = BlockType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Block'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Block" {
            Ok(BlockType)
        } else {
            Err(de::Error::custom("Type not Block"))
        }
    }
}

impl<'de> Deserialize<'de> for BlockType {
    fn deserialize<D>(deserializer: D) -> Result<BlockType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(BlockTypeVisitor)
    }
}
