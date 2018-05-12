use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct DocumentType;

impl Serialize for DocumentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Document")
    }
}

pub struct DocumentTypeVisitor;

impl<'de> Visitor<'de> for DocumentTypeVisitor {
    type Value = DocumentType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Document'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Document" {
            Ok(DocumentType)
        } else {
            Err(de::Error::custom("Type not Document"))
        }
    }
}

impl<'de> Deserialize<'de> for DocumentType {
    fn deserialize<D>(deserializer: D) -> Result<DocumentType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(DocumentTypeVisitor)
    }
}
