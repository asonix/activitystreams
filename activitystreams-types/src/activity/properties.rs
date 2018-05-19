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

//! Namespace for properties of standard Activity types
//!
//! To use these properties in your own types, you can flatten them into your struct with serde:
//!
//! ```rust
//! extern crate activitystreams_traits;
//! extern crate activitystreams_types;
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use activitystreams_traits::{Activity, Object};
//! use activitystreams_types::{
//!   activity::properties::ActivityProperties,
//!   object::properties::ObjectProperties,
//! };
//!
//! #[derive(Clone, Debug, Serialize, Deserialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyActivity {
//!     #[serde(rename = "type")]
//!     pub kind: String,
//!
//!     /// Define a require property for the MyActivity type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     pub object_properties: ObjectProperties,
//!
//!     #[serde(flatten)]
//!     pub activity_properties: ActivityProperties,
//! }
//!
//! impl Object for MyActivity {}
//! impl Activity for MyActivity {}
//! #
//! # fn main() {}
//! ```

use activitystreams_traits::{Link, Object};
use serde_json;

/// Activity objects are specializations of the base Object type that provide information about
/// actions that have either already occurred, are in the process of occurring, or may occur in the
/// future.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActivityProperties {
    /// Describes the result of the activity.
    ///
    /// For instance, if a particular action results in the creation of a new resource, the result
    /// property can be used to describe that new resource.
    ///
    /// - Range: `Object` | `Link`
    /// - Funcitonal: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub result: Option<serde_json::Value>,

    /// Identifies one or more objects used (or to be used) in the completion of an `Activity`.
    ///
    /// - Range: `Object` | `Link`
    /// - Funcitonal: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub instrument: Option<serde_json::Value>,
}

/// Struct with `actor` and optional `origin` and `target` properties
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActorOptOriginAndTarget {
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

    /// Describes an indirect object of the activity from which the activity is directed.
    ///
    /// The precise meaning of the origin is the object of the English preposition "from". For
    /// instance, in the activity "John moved an item to List B from List A", the origin of the
    /// activity is "List A".
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub origin: Option<serde_json::Value>,

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
}

/// Struct with `actor` and `object` properties
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActorAndObject {
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
}

/// Struct with `actor`, `object`, and `target` properties
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActorObjectAndTarget {
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
    #[activitystreams(ab(Object, Link))]
    pub target: serde_json::Value,
}

/// Struct with `actor`, `object`, and optional `target` properties
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActorAndObjectOptTarget {
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
}

/// Struct with `actor`, `object`, and optional `origin` properties
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActorAndObjectOptOrigin {
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

    /// Describes an indirect object of the activity from which the activity is directed.
    ///
    /// The precise meaning of the origin is the object of the English preposition "from". For
    /// instance, in the activity "John moved an item to List B from List A", the origin of the
    /// activity is "List A".
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub origin: Option<serde_json::Value>,
}

/// Struct with `actor`, `object`, and optional `origin` and `target` properties
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActorAndObjectOptOthers {
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

    /// Describes an indirect object of the activity from which the activity is directed.
    ///
    /// The precise meaning of the origin is the object of the English preposition "from". For
    /// instance, in the activity "John moved an item to List B from List A", the origin of the
    /// activity is "List A".
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub origin: Option<serde_json::Value>,

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
}

/// Struct with `actor` and `origin` properties
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ActorAndOrigin {
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

    /// Describes an indirect object of the activity from which the activity is directed.
    ///
    /// The precise meaning of the origin is the object of the English preposition "from". For
    /// instance, in the activity "John moved an item to List B from List A", the origin of the
    /// activity is "List A".
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[activitystreams(ab(Object, Link))]
    pub origin: serde_json::Value,
}

/// Properties for the Accept activity
pub type AcceptProperties = ActorAndObject;

/// Properties for the Add activity
pub type AddProperties = ActorAndObject;

/// Properties for the Move activity
pub type MoveProperties = ActorAndObjectOptOthers;

/// Properties for the Announce activity
pub type AnnounceProperties = ActorAndObjectOptTarget;

/// Properties for the Arrive activity
pub type ArriveProperties = ActorAndOrigin;

/// Properties for the Block activity
pub type BlockProperties = ActorAndObject;

/// Properties for the Create activity
pub type CreateProperties = ActorAndObject;

/// Properties for the Delete activity
pub type DeleteProperties = ActorAndObjectOptOrigin;

/// Properties for the Dislike activity
pub type DislikeProperties = ActorAndObject;

/// Properties for the Flag activity
pub type FlagProperties = ActorAndObject;

/// Properties for the Follow activity
pub type FollowProperties = ActorAndObject;

/// Properties for the Ignore activity
pub type IgnoreProperties = ActorAndObject;

/// Properties for the Invite activity
pub type InviteProperties = ActorObjectAndTarget;

/// Properties for the Join activity
pub type JoinProperties = ActorAndObject;

/// Properties for the Leave activity
pub type LeaveProperties = ActorAndObject;

/// Properties for the Like activity
pub type LikeProperties = ActorAndObject;

/// Properties for the Listen activity
pub type ListenProperties = ActorAndObject;

/// Properties for the Offer activity
pub type OfferProperties = ActorAndObjectOptTarget;

/// Properties for the Question activity
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct QuestionProperties {
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
}

/// Properties for the Read activity
pub type ReadProperties = ActorAndObject;

/// Properties for the Reject activity
pub type RejectProperties = ActorAndObject;

/// Properties for the Remove activity
pub type RemoveProperties = ActorAndObjectOptOthers;

/// Properties for the TentativeAccept activity
pub type TentativeAcceptProperties = ActorAndObject;

/// Properties for the TentativeReject activity
pub type TentativeRejectProperties = ActorAndObject;

/// Properties for the Travel activity
pub type TravelProperties = ActorOptOriginAndTarget;

/// Properties for the Undo activity
pub type UndoProperties = ActorAndObject;

/// Properties for the Update activity
pub type UpdateProperties = ActorAndObject;

/// Properties for the View activity
pub type ViewProperties = ActorAndObject;
