use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use error::{Error, Result};
use link::Link;

mod kind;
mod properties;
pub use self::kind::*;
pub use self::properties::*;

pub trait Object: DeserializeOwned + Serialize {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    #[serde(rename = "type")]
    kind: ArticleType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Article {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    #[serde(rename = "type")]
    kind: AudioType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Audio {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(rename = "type")]
    kind: DocumentType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Document {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    kind: EventType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Event {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    kind: ImageType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Image {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(rename = "type")]
    kind: NoteType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Note {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    #[serde(rename = "type")]
    kind: PageType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Page {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    #[serde(rename = "type")]
    kind: PlaceType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
    #[serde(flatten)]
    pub place: PlaceProperties,
}

impl Object for Place {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(rename = "type")]
    kind: ProfileType,
    describes: serde_json::Value,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Profile {
    pub fn describes<O: Object>(&self) -> Result<O> {
        serde_json::from_value(self.describes.clone()).map_err(|_| Error::Deserialize)
    }
}

impl Object for Profile {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    #[serde(rename = "type")]
    kind: RelationshipType,
    subject: serde_json::Value,
    object: serde_json::Value,
    relationship: serde_json::Value,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Relationship {
    pub fn subject<O: Object>(&self) -> Result<O> {
        serde_json::from_value(self.subject.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn object<O: Object>(&self) -> Result<O> {
        serde_json::from_value(self.object.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn relationship<O: Object>(&self) -> Result<O> {
        serde_json::from_value(self.relationship.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn subject_link<L: Link>(&self) -> Result<L> {
        serde_json::from_value(self.subject.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn object_link<L: Link>(&self) -> Result<L> {
        serde_json::from_value(self.object.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn relationship_link<L: Link>(&self) -> Result<L> {
        serde_json::from_value(self.relationship.clone()).map_err(|_| Error::Deserialize)
    }
}

impl Object for Relationship {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tombstone {
    #[serde(rename = "type")]
    kind: TombstoneType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
    #[serde(flatten)]
    pub tombstone_props: TombstoneProperties,
}

impl Object for Tombstone {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    #[serde(rename = "type")]
    kind: VideoType,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Video {}
