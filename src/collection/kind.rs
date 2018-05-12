use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Collection)]
pub struct CollectionType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(CollectionPage)]
pub struct CollectionPageType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(OrderedCollection)]
pub struct OrderedCollectionType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(OrderedCollectionPage)]
pub struct OrderedCollectionPageType;
