use serde_json;

use object::Object;

mod kind;
mod properties;
pub use self::kind::*;
pub use self::properties::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "@context")]
    context: serde_json::Value,
    #[serde(rename = "type")]
    kind: CollectionType,
    items: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub collection_props: CollectionProperties,
}

impl Object for Collection {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(rename = "@context")]
    context: serde_json::Value,
    #[serde(rename = "type")]
    kind: OrderedCollectionType,
    items: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub collection_props: CollectionProperties,
}

impl Object for OrderedCollection {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPage {
    #[serde(rename = "@context")]
    context: serde_json::Value,
    #[serde(rename = "type")]
    kind: CollectionPageType,
    items: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub collection_props: CollectionProperties,
    #[serde(flatten)]
    pub collection_page_props: CollectionPageProperties,
}

impl Object for CollectionPage {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(rename = "@context")]
    context: serde_json::Value,
    #[serde(rename = "type")]
    kind: OrderedCollectionPageType,
    items: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub collection_props: CollectionProperties,
    #[serde(flatten)]
    pub collection_page_props: CollectionPageProperties,
    #[serde(flatten)]
    pub ordered_collection_page_props: OrderedCollectionPageProperties,
}

impl Object for OrderedCollectionPage {}
