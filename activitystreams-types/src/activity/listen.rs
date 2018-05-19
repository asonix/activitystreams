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

use activitystreams_traits::{Activity, Object};

use super::{
    kind::ListenType, properties::{ActivityProperties, ListenProperties},
};
use object::properties::ObjectProperties;

/// Indicates that the actor has listened to the object.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Listen {
    #[serde(rename = "type")]
    kind: ListenType,

    /// Adds all valid listen properties to this struct
    #[serde(flatten)]
    pub listen_props: ListenProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Listen {}
impl Activity for Listen {}
