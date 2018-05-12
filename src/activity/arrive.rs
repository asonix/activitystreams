use serde_json;

use super::{kind::ArriveType, properties::ActivityProperties, Activity, IntransitiveActivity};

use error::Result;
use link::Link;
use object::{Object, ObjectProperties};
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Arrive {
    #[serde(rename = "type")]
    kind: ArriveType,

    #[activitystreams(ab(Object, Link))]
    actor: serde_json::Value,

    #[activitystreams(ab(Object, Link))]
    location: serde_json::Value,

    #[activitystreams(ab(Object, Link))]
    origin: serde_json::Value,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Arrive {}
impl Activity for Arrive {}
impl IntransitiveActivity for Arrive {}
