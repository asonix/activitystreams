use serde_json;

use super::{kind::TravelType, properties::ActivityProperties, Activity, IntransitiveActivity};
use base::Base;
use error::{Error, Result};
use link::Link;
use object::{Object, ObjectProperties};
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Travel {
    #[serde(rename = "type")]
    kind: TravelType,
    actor: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<serde_json::Value>,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Properties for Travel {}

impl Travel {
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

    pub fn origin<O: Object>(&self) -> Result<O> {
        self.get_item(|d| &d.origin)
    }

    pub fn origins<O: Object>(&self) -> Result<O> {
        self.get_item(|d| &d.origin)
    }

    pub fn origin_link<L: Link>(&self) -> Result<L> {
        self.get_item(|d| &d.origin)
    }

    pub fn origin_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|d| &d.origin)
    }

    pub fn target<O: Object>(&self) -> Result<O> {
        self.get_item(|d| &d.target)
    }

    pub fn targets<O: Object>(&self) -> Result<O> {
        self.get_item(|d| &d.target)
    }

    pub fn target_link<L: Link>(&self) -> Result<L> {
        self.get_item(|d| &d.target)
    }

    pub fn target_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|d| &d.target)
    }
}

impl Base for Travel {}
impl Object for Travel {}
impl Activity for Travel {}
impl IntransitiveActivity for Travel {}
