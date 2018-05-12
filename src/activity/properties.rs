use serde_json;

use error::Result;
use link::Link;
use object::Object;
use Properties;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instrument: Option<serde_json::Value>,
}

impl Properties for ActivityProperties {}

impl ActivityProperties {
    pub fn result<O: Object>(&self) -> Result<O> {
        self.get_item(|ap| &ap.result)
    }

    pub fn results<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|ap| &ap.result)
    }

    pub fn result_link<L: Link>(&self) -> Result<L> {
        self.get_item(|ap| &ap.result)
    }

    pub fn result_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|ap| &ap.result)
    }

    pub fn instrument<O: Object>(&self) -> Result<O> {
        self.get_item(|ap| &ap.instrument)
    }

    pub fn instruments<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|ap| &ap.instrument)
    }

    pub fn instrument_link<L: Link>(&self) -> Result<L> {
        self.get_item(|ap| &ap.instrument)
    }

    pub fn instrument_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|ap| &ap.instrument)
    }
}
