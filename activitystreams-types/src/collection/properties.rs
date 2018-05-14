/*
 * This file is part of ActivityStreams Types.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityStreams Types is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Types is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Types.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Namespace for properties of standard object types
//!
//! To use these properties in your own types, you can flatten them into your struct with serde:
//!
//! ```rust
//! # extern crate activitystreams;
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! #
//! # use activitystreams::{
//! #   collection::properties::CollectionProperties,
//! #   object::properties::ObjectProperties,
//! #   Collection,
//! #   Object
//! # };
//! #
//! #[derive(Clone, Debug, Serialize, Deserialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyCollection {
//!     #[serde(rename = "type")]
//!     pub kind: String,
//!
//!     /// Define a require property for the MyCollection type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     pub object_properties: ObjectProperties,
//!
//!     #[serde(flatten)]
//!     pub collection_properties: CollectionProperties,
//! }
//!
//! impl Object for MyCollection {}
//! impl Collection for MyCollection {}
//! #
//! # fn main() {}
//! ```

use activitystreams_traits::{Collection, CollectionPage, Link, Object};
use serde_json;

/// `Collection` objects are a specialization of the base `Object` that serve as a container for
/// other `Objects` or `Links`.
///
/// The items within a `Collection` can be ordered or unordered. The `OrderedCollection` type MAY be
/// used to identify a `Collection` whose items are always ordered. In the JSON serialization, the
/// unordered items of a `Collection` are represented using the `items` property while ordered items
/// are represented using the `ordered_items` property.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProperties {
    /// Identifies the items contained in a collection. The items might be ordered or unordered.
    ///
    /// - Range: `Object` | `Link` | Ordered List of [ `Object` | `Link` ]
    /// - Functional: false
    #[activitystreams(ab(Object, Link))]
    pub items: serde_json::Value,

    /// A non-negative integer specifying the total number of objects contained by the logical view
    /// of the collection.
    ///
    /// This number might not reflect the actual number of items serialized within the `Collection`
    /// object instance.
    ///
    /// - Range: `xsd:nonNegativeInteger`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    pub total_items: Option<serde_json::Value>,

    /// In a paged `Collection`, indicates the page that contains the most recently updated member
    /// items.
    ///
    /// - Range: `CollectionPage` | `Link`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Link, CollectionPage), functional)]
    pub current: Option<serde_json::Value>,

    /// In a paged `Collection`, indicates the furthest preceeding page of items in the collection.
    ///
    /// - Range: `CollectionPage` | `Link`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Link, CollectionPage), functional)]
    pub first: Option<serde_json::Value>,

    /// In a paged `Collection`, indicates the furthest proceeding page of the collection.
    ///
    /// - Range: `CollectionPage` | `Link`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Link, CollectionPage), functional)]
    pub last: Option<serde_json::Value>,
}

/// The `CollectionPage` type extends from the base `Collection` type and inherits all of it's
/// properties.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct CollectionPageProperties {
    /// Identifies the `Collection` to which a `CollectionPage` objects items belong.
    ///
    /// Range: `Collection` | `Link`
    /// Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Link, Collection), functional)]
    pub part_of: Option<serde_json::Value>,

    /// In a paged `Collection`, indicates the next page of items.
    ///
    /// - Range: `CollectionPage` | `Link`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Link, CollectionPage), functional)]
    pub next: Option<serde_json::Value>,

    /// In a paged `Collection`, identifies the previous page of items.
    ///
    /// - Range: `CollectionPage` | `Link`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Link, CollectionPage), functional)]
    pub prev: Option<serde_json::Value>,
}

/// The OrderedCollectionPage type MAY be used to identify a page whose items are strictly ordered.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPageProperties {
    /// A non-negative integer value identifying the relative position within the logical view of a
    /// strictly ordered collection.
    ///
    /// - Range: `xsd:nonNegativeInteger`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(u64), functional)]
    pub start_index: Option<serde_json::Value>,
}
