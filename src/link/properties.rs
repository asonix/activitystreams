use mime;
use serde_json;

use error::{Error, Result};
use link::Link;
use object::Object;
use Properties;

#[derive(Clone, Debug, Serialize, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct LinkProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    id: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    href: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    rel: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    media_type: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    name: Option<serde_json::Value>,

    // TODO: Lang enum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    hreflang: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    height: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    width: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    preview: Option<serde_json::Value>,
}

impl LinkProperties {
    pub fn media_type(&self) -> Result<mime::Mime> {
        self.media_type_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }
}
