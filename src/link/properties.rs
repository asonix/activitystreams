use mime;
use serde_json;

use error::{Error, Result};
use link::Link;
use object::Object;
use Properties;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkProperties {
    id: Option<serde_json::Value>,
    href: Option<serde_json::Value>,
    rel: Option<serde_json::Value>,
    media_type: Option<serde_json::Value>,
    name: Option<serde_json::Value>,
    hreflang: Option<serde_json::Value>,
    height: Option<serde_json::Value>,
    width: Option<serde_json::Value>,
    preview: Option<serde_json::Value>,
}

impl Properties for LinkProperties {}

impl LinkProperties {
    pub fn id(&self) -> Result<String> {
        self.get_item(|l| &l.id)
    }

    pub fn href(&self) -> Result<String> {
        self.get_item(|l| &l.href)
    }

    pub fn rel(&self) -> Result<String> {
        self.get_item(|l| &l.rel)
    }

    pub fn rels(&self) -> Result<Vec<String>> {
        self.get_item(|l| &l.rel)
    }

    pub fn media_type(&self) -> Result<mime::Mime> {
        self.get_item::<_, String>(|l| &l.media_type)
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }

    pub fn name(&self) -> Result<String> {
        self.get_item(|l| &l.name)
    }

    pub fn names(&self) -> Result<Vec<String>> {
        self.get_item(|l| &l.name)
    }

    // TODO: Lang enum
    pub fn hreflang(&self) -> Result<String> {
        self.get_item(|l| &l.hreflang)
    }

    pub fn height(&self) -> Result<u64> {
        self.get_item(|l| &l.height)
    }

    pub fn width(&self) -> Result<u64> {
        self.get_item(|l| &l.width)
    }

    pub fn preview<O: Object>(&self) -> Result<O> {
        self.get_item(|l| &l.preview)
    }

    pub fn preview_link<L: Link>(&self) -> Result<L> {
        self.get_item(|l| &l.preview)
    }
}
