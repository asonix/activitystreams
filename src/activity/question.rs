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

use serde_json;

use super::{kind::QuestionType, properties::ActivityProperties, Activity, IntransitiveActivity};

use link::Link;
use object::{Object, ObjectProperties};

#[derive(Clone, Debug, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    #[serde(rename = "type")]
    kind: QuestionType,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    #[activitystreams(ab(Object, Link))]
    one_of: Vec<serde_json::Value>,

    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    #[activitystreams(ab(Object, Link))]
    any_of: Vec<serde_json::Value>,

    #[serde(flatten)]
    pub object_props: ObjectProperties,

    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Question {}
impl Activity for Question {}
impl IntransitiveActivity for Question {}
