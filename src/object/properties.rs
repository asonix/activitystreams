use mime;
use serde::de::DeserializeOwned;
use serde_json;

use collection::Collection;
use error::{Error, Result};
use link::Link;
use object::{Image, Object};
use Properties;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributed_to: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audience: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@context")]
    context: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_time: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    generator: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replies: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_time: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tag: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    to: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bto: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<serde_json::Value>,
}

impl Properties for ObjectProperties {}

impl ObjectProperties {
    pub fn id<D: DeserializeOwned>(&self) -> Result<D> {
        self.get_item(|props| &props.id)
    }

    pub fn attachment<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.attachment)
    }

    pub fn attachments<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.attachment)
    }

    pub fn attachment_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.attachment)
    }

    pub fn attachment_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.attachment)
    }

    pub fn attributed_to<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.attributed_to)
    }

    pub fn attributed_tos<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.attributed_to)
    }

    pub fn attributed_to_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.attributed_to)
    }

    pub fn attributed_to_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.attributed_to)
    }

    pub fn audience<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.audience)
    }

    pub fn audiences<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.audience)
    }

    pub fn audience_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.audience)
    }

    pub fn audience_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.audience)
    }

    pub fn content(&self) -> Result<String> {
        self.get_item(|props| &props.content)
    }

    pub fn contents(&self) -> Result<Vec<String>> {
        self.get_item(|props| &props.content)
    }

    pub fn context(&self) -> Result<serde_json::Value> {
        self.context.clone().ok_or(Error::NotFound)
    }

    pub fn name(&self) -> Result<String> {
        self.get_item(|props| &props.name)
    }

    pub fn names(&self) -> Result<Vec<String>> {
        self.get_item(|props| &props.name)
    }

    // TODO: DateTime<Utc>
    pub fn end_time(&self) -> Result<String> {
        self.get_item(|props| &props.end_time)
    }

    pub fn generator<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.generator)
    }

    pub fn generators<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.generator)
    }

    pub fn generator_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.generator)
    }

    pub fn generator_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.generator)
    }

    pub fn icon(&self) -> Result<Image> {
        self.get_item(|props| &props.icon)
    }

    pub fn icons(&self) -> Result<Vec<Image>> {
        self.get_item(|props| &props.icon)
    }

    pub fn icon_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.icon)
    }

    pub fn icon_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.icon)
    }

    pub fn image(&self) -> Result<Image> {
        self.get_item(|props| &props.image)
    }

    pub fn images(&self) -> Result<Vec<Image>> {
        self.get_item(|props| &props.image)
    }

    pub fn image_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.image)
    }

    pub fn image_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.image)
    }

    pub fn in_reply_to<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.in_reply_to)
    }

    pub fn in_reply_tos<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.in_reply_to)
    }

    pub fn in_reply_to_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.in_reply_to)
    }

    pub fn in_reply_to_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.in_reply_to)
    }

    pub fn location<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.location)
    }

    pub fn locations<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.location)
    }

    pub fn location_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.location)
    }

    pub fn location_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.location)
    }

    pub fn preview<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.preview)
    }

    pub fn previews<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.preview)
    }

    pub fn preview_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.preview)
    }

    pub fn preview_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.preview)
    }

    // TODO: DateTime<Utc>
    pub fn published(&self) -> Result<String> {
        self.get_item(|props| &props.published)
    }

    pub fn replies(&self) -> Result<Collection> {
        self.get_item(|props| &props.replies)
    }

    // TODO: DateTime<Utc>
    pub fn start_time(&self) -> Result<String> {
        self.get_item(|props| &props.start_time)
    }

    pub fn summary(&self) -> Result<String> {
        self.get_item(|props| &props.summary)
    }

    pub fn summaries(&self) -> Result<Vec<String>> {
        self.get_item(|props| &props.summary)
    }

    pub fn tag<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.tag)
    }

    pub fn tags<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.tag)
    }

    pub fn tag_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.tag)
    }

    pub fn tag_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.tag)
    }

    // TODO: DateTime<Utc>
    pub fn updated(&self) -> Result<String> {
        self.get_item(|props| &props.updated)
    }

    pub fn url(&self) -> Result<String> {
        self.get_item(|props| &props.url)
    }

    pub fn urls(&self) -> Result<Vec<String>> {
        self.get_item(|props| &props.url)
    }

    pub fn url_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.url)
    }

    pub fn url_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.url)
    }

    pub fn to<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.to)
    }

    pub fn tos<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.to)
    }

    pub fn to_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.to)
    }

    pub fn to_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.to)
    }

    pub fn bto<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.bto)
    }

    pub fn btos<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.bto)
    }

    pub fn bto_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.bto)
    }

    pub fn bto_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.bto)
    }

    pub fn cc<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.cc)
    }

    pub fn ccs<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.cc)
    }

    pub fn cc_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.cc)
    }

    pub fn cc_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.cc)
    }

    pub fn bcc<O: Object>(&self) -> Result<O> {
        self.get_item(|props| &props.bcc)
    }

    pub fn bccs<O: Object>(&self) -> Result<Vec<O>> {
        self.get_item(|props| &props.bcc)
    }

    pub fn bcc_link<L: Link>(&self) -> Result<L> {
        self.get_item(|props| &props.bcc)
    }

    pub fn bcc_links<L: Link>(&self) -> Result<Vec<L>> {
        self.get_item(|props| &props.bcc)
    }

    pub fn media_type(&self) -> Result<mime::Mime> {
        self.get_item::<_, String>(|props| &props.media_type)
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }

    // TODO: xsd:duration
    pub fn duration(&self) -> Result<String> {
        self.get_item(|props| &props.duration)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    altitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    radius: Option<f64>,
}

impl PlaceProperties {
    pub fn accuracy(&self) -> Result<f64> {
        self.accuracy.ok_or(Error::NotFound)
    }

    pub fn altitude(&self) -> Result<f64> {
        self.altitude.ok_or(Error::NotFound)
    }

    pub fn latitude(&self) -> Result<f64> {
        self.latitude.ok_or(Error::NotFound)
    }

    pub fn longitude(&self) -> Result<f64> {
        self.longitude.ok_or(Error::NotFound)
    }

    pub fn radius(&self) -> Result<f64> {
        self.radius.ok_or(Error::NotFound)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TombstoneProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    former_type: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deleted: Option<serde_json::Value>,
}

impl Properties for TombstoneProperties {}

impl TombstoneProperties {
    pub fn former_type(&self) -> Result<String> {
        self.get_item(|t| &t.former_type)
    }

    pub fn former_types(&self) -> Result<Vec<String>> {
        self.get_item(|t| &t.former_type)
    }

    // TODO: DateTime<Utc>
    pub fn deleted(&self) -> Result<String> {
        self.get_item(|t| &t.deleted)
    }
}
