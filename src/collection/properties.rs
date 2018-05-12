use serde_json;

use super::{Collection, CollectionPage};
use error::Result;
use link::Link;
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    total_items: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(CollectionPage), ab(Link), functional)]
    current: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(CollectionPage), ab(Link), functional)]
    first: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(CollectionPage), ab(Link), functional)]
    last: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(Collection), ab(Link), functional)]
    part_of: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(CollectionPage), ab(Link), functional)]
    next: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(CollectionPage), ab(Link), functional)]
    prev: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    start_index: Option<serde_json::Value>,
}
