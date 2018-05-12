use serde_json;

use super::{kind::TravelType, properties::ActivityProperties, Activity, IntransitiveActivity};

use error::Result;
use link::Link;
use object::{Object, ObjectProperties};
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Travel {
    #[serde(rename = "type")]
    kind: TravelType,

    #[activitystreams(ab(Object, Link))]
    actor: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    origin: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    target: Option<serde_json::Value>,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Travel {}
impl Activity for Travel {}
impl IntransitiveActivity for Travel {}
