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

use serde::{de::DeserializeOwned, ser::Serialize};

pub mod kind;
pub mod properties;
use self::kind::*;
use self::properties::*;

/// The Link is the secondary base type for the Activity Streams vocabulary.
pub trait Link: DeserializeOwned + Serialize {}

/// A specialized Link that represents an @mention.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    #[serde(rename = "type")]
    kind: MentionType,

    /// Adds all valid link properties to this struct
    #[serde(flatten)]
    pub link_props: LinkProperties,
}

impl Link for Mention {}
