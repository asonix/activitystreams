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

//! ActivityStreams Types
//!
//! This crate defines the base set of types from the Activity Streams specification.
//!
//! ## Example Usage
//! ```rust
//! extern crate activitystreams_types;
//! extern crate failure;
//! extern crate serde_json;
//!
//! use activitystreams_types::{context, link::Mention};
//! use failure::Error;
//!
//! fn run() -> Result<(), Error> {
//!     /// A Mention is the only predefined Link type in the Activity Streams spec
//!     let mut mention = Mention::default();
//!     mention.link_props.set_context_object(context())?;
//!
//!     let mention_string = serde_json::to_string(&mention)?;
//!
//!     let mention: Mention = serde_json::from_str(&mention_string)?;
//!
//!     Ok(())
//! }
//! #
//! # fn main() {
//! #     run().unwrap();
//! # }
//! ```

#[macro_use]
extern crate activitystreams_derive;
extern crate activitystreams_traits;
extern crate chrono;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

/// Define a simple wrapper around a string for this crate's main Context type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContextObject(pub String);

impl activitystreams_traits::Object for ContextObject {}

/// The context associated with all of the Activity Streams types defined in the crate.
pub fn context() -> ContextObject {
    ContextObject("https://www.w3.org/ns/activitystreams".to_owned())
}

pub mod activity;
pub mod actor;
pub mod collection;
mod custom_props;
pub mod link;
pub mod object;

pub use self::custom_props::{CustomLink, CustomObject};
