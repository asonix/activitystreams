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
    kind::RemoveType,
    properties::{ActivityProperties, RemoveProperties},
    ActivityExt,
};
use object::{properties::ObjectProperties, ObjectExt};

/// Indicates that the actor is removing the object.
///
/// If specified, the origin indicates the context from which the object is being removed.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Remove {
    #[serde(rename = "type")]
    pub kind: RemoveType,

    /// Adds all valid remove properties to this struct
    #[serde(flatten)]
    pub remove_props: RemoveProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Remove {}
impl ObjectExt for Remove {
    fn props(&self) -> &ObjectProperties {
        &self.object_props
    }

    fn props_mut(&mut self) -> &mut ObjectProperties {
        &mut self.object_props
    }
}
impl Activity for Remove {}
impl ActivityExt for Remove {
    fn props(&self) -> &ActivityProperties {
        &self.activity_props
    }

    fn props_mut(&mut self) -> &mut ActivityProperties {
        &mut self.activity_props
    }
}
