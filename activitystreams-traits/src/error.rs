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

use std::result;

/// The Error type
#[derive(Copy, Clone, Debug, Eq, Fail, PartialEq)]
pub enum Error {
    /// This error occurs when an Activity Streams type does not contain a requested value
    #[fail(display = "Key not present")]
    NotFound,

    /// This error occurs when a requested value could not be deserialized into the requested type
    #[fail(display = "Failed to deserialize data as requested type")]
    Deserialize,

    /// This error occurs when a provided item could not be serialized into an Activity Streams
    /// type
    #[fail(display = "Failed to serialize data")]
    Serialize,
}

/// An alias for Result<T, Error>
pub type Result<T> = result::Result<T, Error>;
