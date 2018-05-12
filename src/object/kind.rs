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
#[activitystreams(Article)]
pub struct ArticleType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Audio)]
pub struct AudioType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Document)]
pub struct DocumentType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Event)]
pub struct EventType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Image)]
pub struct ImageType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Note)]
pub struct NoteType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Page)]
pub struct PageType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Place)]
pub struct PlaceType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Profile)]
pub struct ProfileType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Relationship)]
pub struct RelationshipType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Tombstone)]
pub struct TombstoneType;

#[derive(Clone, Debug, UnitString)]
#[activitystreams(Video)]
pub struct VideoType;
