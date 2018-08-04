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
    kind::LeaveType,
    properties::{ActivityProperties, LeaveProperties},
    ActivityExt,
};
use object::{properties::ObjectProperties, ObjectExt};

/// Indicates that the actor has left the object.
///
/// The target and origin typically have no meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Leave {
    #[serde(rename = "type")]
    pub kind: LeaveType,

    /// Adds all valid leave properties to this struct
    #[serde(flatten)]
    pub leave_props: LeaveProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Leave {}
impl ObjectExt for Leave {
    fn props(&self) -> &ObjectProperties {
        &self.object_props
    }

    fn props_mut(&mut self) -> &mut ObjectProperties {
        &mut self.object_props
    }
}
impl Activity for Leave {}
impl ActivityExt for Leave {
    fn props(&self) -> &ActivityProperties {
        &self.activity_props
    }

    fn props_mut(&mut self) -> &mut ActivityProperties {
        &mut self.activity_props
    }
}
