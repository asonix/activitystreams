use serde_json;

use super::{Collection, CollectionPage};
use error::Result;
use link::Link;
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    total_items: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    current: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    first: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last: Option<serde_json::Value>,
}

impl Properties for CollectionProperties {}

impl CollectionProperties {
    pub fn total_items(&self) -> Result<u64> {
        self.get_item(|c| &c.total_items)
    }

    pub fn current(&self) -> Result<CollectionPage> {
        self.get_item(|c| &c.current)
    }

    pub fn current_link<L: Link>(&self) -> Result<L> {
        self.get_item(|c| &c.current)
    }

    pub fn first(&self) -> Result<CollectionPage> {
        self.get_item(|c| &c.first)
    }

    pub fn first_link<L: Link>(&self) -> Result<L> {
        self.get_item(|c| &c.first)
    }

    pub fn last(&self) -> Result<CollectionPage> {
        self.get_item(|c| &c.last)
    }

    pub fn last_link<L: Link>(&self) -> Result<L> {
        self.get_item(|c| &c.last)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    part_of: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prev: Option<serde_json::Value>,
}

impl Properties for CollectionPageProperties {}

impl CollectionPageProperties {
    pub fn part_of(&self) -> Result<Collection> {
        self.get_item(|c| &c.part_of)
    }

    pub fn part_of_link<L: Link>(&self) -> Result<L> {
        self.get_item(|c| &c.part_of)
    }

    pub fn next(&self) -> Result<CollectionPage> {
        self.get_item(|c| &c.next)
    }

    pub fn next_link<L: Link>(&self) -> Result<L> {
        self.get_item(|c| &c.next)
    }

    pub fn prev(&self) -> Result<CollectionPage> {
        self.get_item(|c| &c.prev)
    }

    pub fn prev_link<L: Link>(&self) -> Result<L> {
        self.get_item(|c| &c.prev)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_index: Option<serde_json::Value>,
}

impl Properties for OrderedCollectionPageProperties {}

impl OrderedCollectionPageProperties {
    pub fn start_index(&self) -> Result<u64> {
        self.get_item(|c| &c.start_index)
    }
}
