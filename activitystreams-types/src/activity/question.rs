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

use activitystreams_traits::{Activity, IntransitiveActivity, Link, Object};
use serde_json;

use super::{kind::QuestionType, properties::ActivityProperties};
use object::properties::ObjectProperties;

/// Represents a question being asked.
///
/// Question objects are an extension of IntransitiveActivity. That is, the Question object is an
/// Activity, but the direct object is the question itself and therefore it would not contain an
/// object property.
///
/// Either of the anyOf and oneOf properties MAY be used to express possible answers, but a
/// Question object MUST NOT have both properties.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    #[serde(rename = "type")]
    pub kind: QuestionType,

    /// Identifies an exclusive option for a Question.
    ///
    /// Use of `one_of` implies that the Question can have only a single answer. To indicate that a
    /// `Question` can have multiple answers, use `any_of`.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub one_of: Option<serde_json::Value>,

    /// Identifies an inclusive option for a Question.
    ///
    /// Use of `any_of` implies that the Question can have multiple answers. To indicate that a
    /// `Question` can have only one answer, use `one_of`.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub any_of: Option<serde_json::Value>,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Question {}
impl Activity for Question {}
impl IntransitiveActivity for Question {}
