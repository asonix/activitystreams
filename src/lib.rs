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

//! ActivityStreams
//!
//! A set of Traits and Types that make up the Activity Streams specification
//!
//! ## Examples
//!
//! ### Basic
//!
//! ```rust
//! extern crate activitystreams;
//! extern crate failure;
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//!
//! use activitystreams::{context, Object, Actor, object::Profile};
//! use failure::Error;
//!
//! #[derive(Clone, Debug, Default, Deserialize, Serialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct Persona {
//!     #[serde(rename = "@context")]
//!     context: serde_json::Value,
//!
//!     #[serde(rename = "type")]
//!     kind: String,
//! }
//!
//! impl Object for Persona {}
//! impl Actor for Persona {}
//!
//! fn run() -> Result<(), Error> {
//!     let mut profile = Profile::default();
//!
//!     profile.profile.set_describes_object(Persona {
//!         context: serde_json::to_value(context())?,
//!
//!         kind: "Persona".to_owned(),
//!     })?;
//!
//!     profile.object_props.set_context_object(context())?;
//!
//!     let profile_string = serde_json::to_string(&profile)?;
//!
//!     let profile: Profile = serde_json::from_str(&profile_string)?;
//!
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #   run().unwrap();
//! # }
//! ```
//!
//! ### Advanced
//!
//! ```rust
//! #[macro_use]
//! extern crate activitystreams_derive;
//! extern crate activitystreams_traits;
//! extern crate activitystreams_types;
//! extern crate failure;
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//!
//! use activitystreams_traits::{Link, Object};
//! use activitystreams_types::{CustomLink, link::Mention};
//! use failure::Error;
//!
//! /// Using the UnitString derive macro
//! ///
//! /// This macro implements Serialize and Deserialize for the given type, making this type
//! /// represent the string "SomeKind" in JSON.
//! #[derive(Clone, Debug, Default, UnitString)]
//! #[activitystreams(SomeKind)]
//! pub struct MyKind;
//!
//! /// Using the Properties derive macro
//! ///
//! /// This macro generates getters and setters for the associated fields.
//! #[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyProperties {
//!     /// Derive getters and setters for @context with Link and Object traits.
//!     #[serde(rename = "@context")]
//!     #[activitystreams(ab(Object, Link))]
//!     pub context: Option<serde_json::Value>,
//!
//!     /// Use the UnitString MyKind to enforce the type of the object by "SomeKind"
//!     pub kind: MyKind,
//!
//!     /// Derive getters and setters for required_key with String type.
//!     ///
//!     /// In the Activity Streams spec, 'functional' means there can only be one item for this
//!     /// key. This means all fields not labeled 'functional' can also be serialized/deserialized
//!     /// as Vec<T>.
//!     #[activitystreams(concrete(String), functional)]
//!     pub required_key: serde_json::Value,
//! }
//!
//! fn run() -> Result<(), Error> {
//!     let mut props = MyProperties::default();
//!
//!     props.set_required_key_string("Hey".to_owned())?;
//!
//!     let my_link = CustomLink::new(Mention::default(), props);
//!
//!     let my_link_string = serde_json::to_string(&my_link)?;
//!
//!     let my_link: CustomLink<Mention, MyProperties> = serde_json::from_str(&my_link_string)?;
//!
//!     Ok(())
//! }
//! # fn main() {
//! #   run().unwrap();
//! # }
//! ```

extern crate activitystreams_traits;
extern crate activitystreams_types;

pub mod activity;
pub mod actor;
pub mod collection;
mod error;
pub mod link;
pub mod object;

pub use self::activity::{Activity, ActivityExt, IntransitiveActivity};
pub use self::actor::Actor;
pub use self::collection::{Collection, CollectionExt, CollectionPage, CollectionPageExt};
pub use self::error::{Error, Result};
pub use self::link::{Link, LinkExt};
pub use self::object::{Object, ObjectExt};
pub use activitystreams_traits::properties;
pub use activitystreams_types::context;
