use serde_json;

use super::{kind::ArriveType, properties::ActivityProperties, Activity, IntransitiveActivity};

use error::{Error, Result};
use link::Link;
use object::{Object, ObjectProperties};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Arrive {
    #[serde(rename = "type")]
    kind: ArriveType,
    actor: serde_json::Value,
    location: serde_json::Value,
    origin: serde_json::Value,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Arrive {
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

    pub fn location<O: Object>(&self) -> Result<O> {
        serde_json::from_value(self.location.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn locations<O: Object>(&self) -> Result<Vec<O>> {
        serde_json::from_value(self.location.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn location_link<L: Link>(&self) -> Result<L> {
        serde_json::from_value(self.location.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn location_links<L: Link>(&self) -> Result<Vec<L>> {
        serde_json::from_value(self.location.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn origin<O: Object>(&self) -> Result<O> {
        serde_json::from_value(self.origin.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn origins<O: Object>(&self) -> Result<Vec<O>> {
        serde_json::from_value(self.origin.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn origin_link<L: Link>(&self) -> Result<L> {
        serde_json::from_value(self.origin.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn origin_links<L: Link>(&self) -> Result<Vec<L>> {
        serde_json::from_value(self.origin.clone()).map_err(|_| Error::Deserialize)
    }
}

impl Object for Arrive {}
impl Activity for Arrive {}
impl IntransitiveActivity for Arrive {}
