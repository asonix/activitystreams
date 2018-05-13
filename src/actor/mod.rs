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

use object::{Object, ObjectProperties};

mod kind;
pub use self::kind::*;

pub trait Actor: Object {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Appliation {
    #[serde(rename = "type")]
    kind: ApplicationType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Appliation {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "type")]
    kind: GroupType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Group {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Organization {
    #[serde(rename = "type")]
    kind: OrganizationType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Organization {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[serde(rename = "type")]
    kind: PersonType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Person {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(rename = "type")]
    kind: ServiceType,

    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Service {}
