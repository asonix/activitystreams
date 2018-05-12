use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Article)]
pub struct ArticleType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Audio)]
pub struct AudioType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Document)]
pub struct DocumentType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Event)]
pub struct EventType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Image)]
pub struct ImageType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Note)]
pub struct NoteType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Page)]
pub struct PageType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Place)]
pub struct PlaceType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Profile)]
pub struct ProfileType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Relationship)]
pub struct RelationshipType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Tombstone)]
pub struct TombstoneType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Video)]
pub struct VideoType;
