/*
 * This file is part of ActivityStreams Traits.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityStreams Traits is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Traits is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Traits.  If not, see <http://www.gnu.org/licenses/>.
 */

#[macro_use]
extern crate failure;
extern crate serde;
extern crate serde_json;

mod activity;
mod actor;
mod collection;
mod error;
mod link;
mod object;
pub mod properties;

pub use self::activity::*;
pub use self::actor::*;
pub use self::collection::*;
pub use self::error::*;
pub use self::link::*;
pub use self::object::*;