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

#[macro_use]
extern crate activitystreams_derive;
extern crate chrono;
#[macro_use]
extern crate failure;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub fn context() -> serde_json::Value {
    json!({
        "one": "two",
    })
}

pub mod activity;
pub mod actor;
pub mod collection;
pub mod custom_props;
pub mod error;
pub mod link;
pub mod object;
pub mod properties;

pub use self::activity::{Activity, IntransitiveActivity};
pub use self::actor::Actor;
pub use self::custom_props::{CustomLink, CustomObject};
pub use self::error::Error;
pub use self::link::Link;
pub use self::object::Object;
