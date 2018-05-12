use base::Base;

mod kind;
mod properties;
pub use self::kind::*;
pub use self::properties::*;

pub trait Link: Base {}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    #[serde(rename = "type")]
    kind: MentionType,
    #[serde(flatten)]
    pub link_props: LinkProperties,
}

impl Base for Mention {}
impl Link for Mention {}
