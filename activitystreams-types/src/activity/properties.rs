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

//! Namespace for properties of standard Activity types
//!
//! To use these properties in your own types, you can flatten them into your struct with serde:
//!
//! ```rust
//! extern crate activitystreams_traits;
//! extern crate activitystreams_types;
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use activitystreams_traits::{Activity, Object};
//! use activitystreams_types::{
//!   activity::properties::ActivityProperties,
//!   object::properties::ObjectProperties,
//! };
//!
//! #[derive(Clone, Debug, Serialize, Deserialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyActivity {
//!     #[serde(rename = "type")]
//!     pub kind: String,
//!
//!     /// Define a require property for the MyActivity type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     pub object_properties: ObjectProperties,
//!
//!     #[serde(flatten)]
//!     pub activity_properties: ActivityProperties,
//! }
//!
//! impl Object for MyActivity {}
//! impl Activity for MyActivity {}
//! #
//! # fn main() {}
//! ```

use activitystreams_traits::{Link, Object};
use serde_json;

/// Activity objects are specializations of the base Object type that provide information about
/// actions that have either already occurred, are in the process of occurring, or may occur in the
/// future.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActivityProperties {
    /// Describes the result of the activity.
    ///
    /// For instance, if a particular action results in the creation of a new resource, the result
    /// property can be used to describe that new resource.
    ///
    /// - Range: `Object` | `Link`
    /// - Funcitonal: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub result: Option<serde_json::Value>,

    /// Identifies one or more objects used (or to be used) in the completion of an `Activity`.
    ///
    /// - Range: `Object` | `Link`
    /// - Funcitonal: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub instrument: Option<serde_json::Value>,
}
