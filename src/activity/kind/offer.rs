use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug)]
pub struct OfferType;

impl Serialize for OfferType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("Offer")
    }
}

pub struct OfferTypeVisitor;

impl<'de> Visitor<'de> for OfferTypeVisitor {
    type Value = OfferType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "The string 'Offer'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v == "Offer" {
            Ok(OfferType)
        } else {
            Err(de::Error::custom("Type not Offer"))
        }
    }
}

impl<'de> Deserialize<'de> for OfferType {
    fn deserialize<D>(deserializer: D) -> Result<OfferType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OfferTypeVisitor)
    }
}
