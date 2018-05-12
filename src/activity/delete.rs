use serde_json;

use super::{kind::DeleteType, properties::ActivityProperties, Activity};

use error::Result;
use link::Link;
use object::{Object, ObjectProperties};
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Delete {
    #[serde(rename = "type")]
    kind: DeleteType,

    #[activitystreams(ab(Object, Link))]
    actor: serde_json::Value,

    #[activitystreams(ab(Object, Link))]
    object: serde_json::Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    origin: Option<serde_json::Value>,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Delete {}
impl Activity for Delete {}
