use chrono::{offset::Utc, DateTime};
use mime;
use serde_json::{self, Value};

use collection::Collection;
use error::{Error, Result};
use link::Link;
use object::{Image, Object};
use Properties;

#[derive(Clone, Debug, Serialize, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ObjectProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    id: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    attachment: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    attributed_to: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    audience: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    content: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "@context")]
    #[activitystreams(concrete(Value), functional)]
    context: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    name: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    end_time: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    generator: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Link), concrete(Image))]
    icon: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Link), concrete(Image))]
    image: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    in_reply_to: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    location: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    preview: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    published: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(Collection), functional)]
    replies: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    start_time: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    summary: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    tag: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    updated: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), ab(Link))]
    url: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    to: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    bto: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    cc: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    bcc: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    media_type: Option<serde_json::Value>,

    // TODO: xsd:duration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    duration: Option<serde_json::Value>,
}

impl ObjectProperties {
    pub fn media_type(&self) -> Result<mime::Mime> {
        self.media_type_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }

    pub fn end_time(&self) -> Result<DateTime<Utc>> {
        self.end_time_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }

    pub fn published(&self) -> Result<DateTime<Utc>> {
        self.published_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }

    pub fn start_time(&self) -> Result<DateTime<Utc>> {
        self.start_time_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }

    pub fn updated(&self) -> Result<DateTime<Utc>> {
        self.updated_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct PlaceProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    accuracy: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    altitude: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    latitude: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    longitude: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    radius: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct TombstoneProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    former_type: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    deleted: Option<serde_json::Value>,
}

impl TombstoneProperties {
    pub fn deleted(&self) -> Result<DateTime<Utc>> {
        self.deleted_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }
}
