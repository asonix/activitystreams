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

use serde_json;

use super::{Collection, CollectionPage};
use link::Link;

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPageProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    start_index: Option<serde_json::Value>,
}
