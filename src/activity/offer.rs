use serde_json;

use super::{kind::OfferType, properties::ActivityProperties, Activity};

use error::{Error, Result};
use link::Link;
use object::{Object, ObjectProperties};
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    #[serde(rename = "type")]
    kind: OfferType,
    actor: serde_json::Value,
    object: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<serde_json::Value>,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Properties for Offer {}

impl Offer {
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

impl Object for Offer {}
impl Activity for Offer {}
