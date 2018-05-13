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

use std::fmt;

use serde::{
    de::{self, Deserialize, Deserializer, Visitor}, ser::{Serialize, Serializer},
};

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Application)]
pub struct ApplicationType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Group)]
pub struct GroupType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Organization)]
pub struct OrganizationType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Person)]
pub struct PersonType;

#[derive(Clone, Debug, Default, UnitString)]
#[activitystreams(Service)]
pub struct ServiceType;
