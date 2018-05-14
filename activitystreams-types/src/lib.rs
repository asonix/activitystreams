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

#[macro_use]
extern crate activitystreams_derive;
extern crate activitystreams_traits;
extern crate chrono;
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
mod custom_props;
pub mod link;
pub mod object;

pub use self::custom_props::{CustomLink, CustomObject};
