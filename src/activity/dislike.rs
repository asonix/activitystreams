use serde_json;

use super::{kind::DislikeType, properties::ActivityProperties, Activity};

use error::Result;
use link::Link;
use object::{Object, ObjectProperties};
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Dislike {
    #[serde(rename = "type")]
    kind: DislikeType,

    #[activitystreams(ab(Object, Link))]
    actor: serde_json::Value,

    #[activitystreams(ab(Object, Link))]
    object: serde_json::Value,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Dislike {}
impl Activity for Dislike {}
