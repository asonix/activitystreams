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

use link::Link;
use object::Object;

mod kind;
mod properties;
pub use self::kind::*;
pub use self::properties::*;

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
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
