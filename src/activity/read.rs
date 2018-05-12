use serde_json;

use super::{kind::ReadType, properties::ActivityProperties, Activity};
use base::Base;
use error::{Error, Result};
use link::Link;
use object::{Object, ObjectProperties};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Read {
    #[serde(rename = "type")]
    kind: ReadType,
    actor: serde_json::Value,
    object: serde_json::Value,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Read {
    pub fn actor<O: Object>(&self) -> Result<O> {
        serde_json::from_value(self.actor.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn actors<O: Object>(&self) -> Result<Vec<O>> {
        serde_json::from_value(self.actor.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn actor_link<L: Link>(&self) -> Result<L> {
        serde_json::from_value(self.actor.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn actor_links<L: Link>(&self) -> Result<Vec<L>> {
        serde_json::from_value(self.actor.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn object<O: Object>(&self) -> Result<O> {
        serde_json::from_value(self.object.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn objects<O: Object>(&self) -> Result<Vec<O>> {
        serde_json::from_value(self.object.clone()).map_err(|_| Error::Deserialize)
    }
}

impl Base for Read {}
impl Object for Read {}
impl Activity for Read {}
