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

use mime;
use serde_json;

use error::{Error, Result};
use link::Link;
use object::Object;

#[derive(Clone, Debug, Serialize, Deserialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct LinkProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    id: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    href: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    rel: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    media_type: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    name: Option<serde_json::Value>,

    // TODO: Lang enum
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    hreflang: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    height: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    width: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    preview: Option<serde_json::Value>,
}

impl LinkProperties {
    pub fn media_type(&self) -> Result<mime::Mime> {
        self.media_type_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }
}
