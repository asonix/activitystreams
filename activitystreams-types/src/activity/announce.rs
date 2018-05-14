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

use activitystreams_traits::{Activity, Link, Object};
use serde_json;

use super::{kind::AnnounceType, properties::ActivityProperties};
use object::properties::ObjectProperties;

/// Indicates that the actor is calling the target's attention the object.
///
/// The origin typically has no defined meaning.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Announce {
    #[serde(rename = "type")]
    pub kind: AnnounceType,

    /// Describes one or more entities that either performed or are expected to perform the
    /// activity.
    ///
    /// Any single activity can have multiple actors. The actor MAY be specified using an indirect
    /// Link.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[activitystreams(ab(Object, Link))]
    pub actor: serde_json::Value,

    /// When used within an Activity, describes the direct object of the activity.
    ///
    /// For instance, in the activity "John added a movie to his wishlist", the object of the
    /// activity is the movie added.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[activitystreams(ab(Object, Link))]
    pub object: serde_json::Value,

    /// Describes the indirect object, or target, of the activity.
    ///
    /// The precise meaning of the target is largely dependent on the type of action being
    /// described but will often be the object of the English preposition "to". For instance, in
    /// the activity "John added a movie to his wishlist", the target of the activity is John's
    /// wishlist. An activity can have more than one target
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub target: Option<serde_json::Value>,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid activity properties to this struct
    #[serde(flatten)]
    pub activity_props: ActivityProperties,
}

impl Object for Announce {}
impl Activity for Announce {}
