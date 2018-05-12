use serde_json;

use super::{kind::QuestionType, properties::ActivityProperties, Activity, IntransitiveActivity};

use error::Result;
use link::Link;
use object::{Object, ObjectProperties};
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    #[serde(rename = "type")]
    kind: QuestionType,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    #[activitystreams(ab(Object, Link))]
    one_of: Vec<serde_json::Value>,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    #[activitystreams(ab(Object, Link))]
    any_of: Vec<serde_json::Value>,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Question {}
impl Activity for Question {}
impl IntransitiveActivity for Question {}
