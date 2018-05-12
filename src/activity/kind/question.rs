use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct QuestionType;

impl Serialize for QuestionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Question")
    }
}

pub struct QuestionTypeVisitor;

impl<'de> Visitor<'de> for QuestionTypeVisitor {
    type Value = QuestionType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Question'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Question" {
            Ok(QuestionType)
        } else {
            Err(de::Error::custom("Type not Question"))
        }
    }
}

impl<'de> Deserialize<'de> for QuestionType {
    fn deserialize<D>(deserializer: D) -> Result<QuestionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(QuestionTypeVisitor)
    }
}
