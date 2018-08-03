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

use activitystreams_traits::{Activity, IntransitiveActivity, Object};

use super::{
    kind::QuestionType,
    properties::{ActivityExt, ActivityProperties, QuestionProperties},
};
use object::properties::{ObjectExt, ObjectProperties};

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

    /// Adds all valid question properties to this struct
    #[serde(flatten)]
    pub question_props: QuestionProperties,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Question {}
impl ObjectExt for Question {
    fn props(&self) -> &ObjectProperties {
        &self.object_props
    }

    fn props_mut(&mut self) -> &mut ObjectProperties {
        &mut self.object_props
    }
}
impl Activity for Question {}
impl ActivityExt for Question {
    fn props(&self) -> &ActivityProperties {
        &self.activity_props
    }

    fn props_mut(&mut self) -> &mut ActivityProperties {
        &mut self.activity_props
    }
}
impl IntransitiveActivity for Question {}
