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

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Collection)]
pub struct CollectionType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(CollectionPage)]
pub struct CollectionPageType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(OrderedCollection)]
pub struct OrderedCollectionType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(OrderedCollectionPage)]
pub struct OrderedCollectionPageType;
