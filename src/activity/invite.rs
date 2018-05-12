use serde_json;

use super::{kind::InviteType, properties::ActivityProperties, Activity};
use base::Base;
use error::{Error, Result};
use link::Link;
use object::{Object, ObjectProperties};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invite {
    #[serde(rename = "type")]
    kind: InviteType,
    actor: serde_json::Value,
    object: serde_json::Value,
    target: serde_json::Value,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Invite {
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
        serde_json::from_value(self.target.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn targets<O: Object>(&self) -> Result<Vec<O>> {
        serde_json::from_value(self.target.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn target_link<L: Link>(&self) -> Result<L> {
        serde_json::from_value(self.target.clone()).map_err(|_| Error::Deserialize)
    }

    pub fn target_links<L: Link>(&self) -> Result<Vec<L>> {
        serde_json::from_value(self.target.clone()).map_err(|_| Error::Deserialize)
    }
}

impl Base for Invite {}
impl Object for Invite {}
impl Activity for Invite {}
