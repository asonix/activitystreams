use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct ArticleType;

impl Serialize for ArticleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Article")
    }
}

pub struct ArticleTypeVisitor;

impl<'de> Visitor<'de> for ArticleTypeVisitor {
    type Value = ArticleType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Article'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Article" {
            Ok(ArticleType)
        } else {
            Err(de::Error::custom("Type not Article"))
        }
    }
}

impl<'de> Deserialize<'de> for ArticleType {
    fn deserialize<D>(deserializer: D) -> Result<ArticleType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ArticleTypeVisitor)
    }
}
