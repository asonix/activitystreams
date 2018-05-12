use serde_json;

use error::Result;
use link::Link;
use object::Object;

mod kind;
mod properties;
pub use self::kind::*;
pub use self::properties::*;
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "type")]
    kind: CollectionType,

    #[activitystreams(ab(Object, Link))]
    items: Vec<serde_json::Value>,

    #[serde(flatten)]
    pub collection_props: CollectionProperties,
}

impl Object for Collection {}

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(rename = "type")]
    kind: OrderedCollectionType,

    #[activitystreams(ab(Object, Link))]
    items: Vec<serde_json::Value>,

    #[serde(flatten)]
    pub collection_props: CollectionProperties,
}

impl Object for OrderedCollection {}

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage {
    #[serde(rename = "type")]
    kind: CollectionPageType,

    #[activitystreams(ab(Object, Link))]
    items: Vec<serde_json::Value>,

    #[serde(flatten)]
    pub collection_props: CollectionProperties,

    #[serde(flatten)]
    pub collection_page_props: CollectionPageProperties,
}

impl Object for CollectionPage {}

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(rename = "type")]
    kind: OrderedCollectionPageType,

    #[activitystreams(ab(Object, Link))]
    items: Vec<serde_json::Value>,

    #[serde(flatten)]
    pub collection_props: CollectionProperties,

    #[serde(flatten)]
    pub collection_page_props: CollectionPageProperties,

    #[serde(flatten)]
    pub ordered_collection_page_props: OrderedCollectionPageProperties,
}

impl Object for OrderedCollectionPage {}
