/*
 * This file is part of ActivityStreams.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityStreams is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams.  If not, see <http://www.gnu.org/licenses/>.
 */

use chrono::{offset::Utc, DateTime};
use mime;
use serde_json::{self, Value};

use collection::Collection;
use error::{Error, Result};
use link::Link;
use object::{Image, Object};

pub type UtcTime = DateTime<Utc>;

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
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
    #[activitystreams(concrete(String, UtcTime), functional)]
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
    #[activitystreams(concrete(String, UtcTime), functional)]
    published: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(Collection), functional)]
    replies: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String, UtcTime), functional)]
    start_time: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    summary: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    tag: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String, UtcTime), functional)]
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
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct TombstoneProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    former_type: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String, UtcTime), functional)]
    deleted: Option<serde_json::Value>,
}
