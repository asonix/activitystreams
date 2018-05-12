use serde::de::DeserializeOwned;
use serde_json;

use super::{kind::QuestionType, properties::ActivityProperties, Activity, IntransitiveActivity};
use base::Base;
use error::{Error, Result};
use link::Link;
use object::{Object, ObjectProperties};
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    #[serde(rename = "type")]
    kind: QuestionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    one_of: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    any_of: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub object_props: ObjectProperties,
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Properties for Question {}

impl Question {
    pub fn one_of<O: Object>(&self) -> Result<Vec<O>> {
        vec_item(&self.one_of)
    }

    pub fn one_of_link<L: Link>(&self) -> Result<Vec<L>> {
        vec_item(&self.one_of)
    }

    pub fn any_of<O: Object>(&self) -> Result<Vec<O>> {
        vec_item(&self.any_of)
    }

    pub fn any_of_link<L: Link>(&self) -> Result<Vec<L>> {
        vec_item(&self.any_of)
    }
}
fn vec_item<D: DeserializeOwned>(v: &Option<Vec<serde_json::Value>>) -> Result<Vec<D>> {
    if let Some(v) = v.clone() {
        v.into_iter()
            .map(|value| serde_json::from_value(value))
            .fold(Ok(Vec::new()), |acc, item| match acc {
                Ok(mut v) => match item {
                    Ok(item) => {
                        v.push(item);
                        Ok(v)
                    }
                    Err(_) => Err(Error::Deserialize),
                },
                Err(e) => Err(e),
            })
    } else {
        Err(Error::NotFound)
    }
}

impl Base for Question {}
impl Object for Question {}
impl Activity for Question {}
impl IntransitiveActivity for Question {}
