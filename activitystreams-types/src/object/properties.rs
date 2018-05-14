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

//! Namespace for properties of standard object types
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
//! use activitystreams_traits::Object;
//! use activitystreams_types::object::properties::ObjectProperties;
//!
//! #[derive(Clone, Debug, Serialize, Deserialize)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyObject {
//!     #[serde(rename = "type")]
//!     pub kind: String,
//!
//!     /// Define a require property for the MyObject type
//!     pub my_property: String,
//!
//!     #[serde(flatten)]
//!     pub object_properties: ObjectProperties,
//! }
//!
//! impl Object for MyObject {}
//! #
//! # fn main() {}
//! ```

use activitystreams_traits::{Collection, Error, Link, Object, Result};
use chrono::{offset::Utc, DateTime};
use mime;
use serde_json;

use object::Image;

/// Alias chrono::DateTime<Utc> for use in derive macros
pub type UtcTime = DateTime<Utc>;

/// Define all the properties of the Object base type as described by the Activity Streams
/// vocabulary.
///
/// In addition to having a global identifier (expressed as an absolute IRI using the id property)
/// and an "object type" (expressed using the type property), all instances of the Object type share
/// a common set of properties normatively defined by the Activity Vocabulary.
///
/// This struct does not provide an optional `type` property, if you are implementing your own
/// object type, you must supply your own type. This crate's provided object types all supply their
/// own `type` properties as Unit Structs with custom serde behaviour.
///
/// All properties are optional (including the id and type).
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ObjectProperties {
    // TODO: IRI type
    /// Provides the globally unique identifier for an Object or Link.
    ///
    /// The `id` property is expressed as an absolute IRI in the spec, but for now is represented
    /// as a string.
    ///
    /// - Range: `anyUri`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    pub id: Option<serde_json::Value>,

    /// Identifies a resource attached or related to an object that potentially requires special
    /// handling.
    ///
    /// The intent is to provide a model that is at least semantically similar to attachments in
    /// email.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub attachment: Option<serde_json::Value>,

    /// Identifies one or more entities to which this object is attributed.
    ///
    /// The attributed entities might not be Actors. For instance, an object might be attributed to
    /// the completion of another activity.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub attributed_to: Option<serde_json::Value>,

    /// Identifies one or more entities that represent the total population of entities for which
    /// the object can considered to be relevant.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub audience: Option<serde_json::Value>,

    // TODO: rdf:langString
    /// The content or textual representation of the Object encoded as a JSON string.
    ///
    /// By default, the value of content is HTML. The mediaType property can be used in the object
    /// to indicate a different content type.
    ///
    /// The content MAY be expressed using multiple language-tagged values.
    ///
    /// - Range: `xsd:string` | `rdf:langString`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    pub content: Option<serde_json::Value>,

    /// Identifies the context within which the object exists or an activity was performed.
    ///
    /// The notion of "context" used is intentionally vague. The intended function is to serve as a
    /// means of grouping objects and activities that share a common originating context or purpose.
    /// An example could be all activities relating to a common project or event.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none", rename = "@context")]
    #[activitystreams(ab(Object, Link))]
    pub context: Option<serde_json::Value>,

    // TODO: rdf:langString
    /// A simple, human-readable, plain-text name for the object.
    ///
    /// HTML markup MUST NOT be included. The name MAY be expressed using multiple language-tagged
    /// values.
    ///
    /// - Range: `xsd:string` | `rdf:langString`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    pub name: Option<serde_json::Value>,

    /// The date and time describing the actual or expected ending time of the object.
    ///
    /// When used with an Activity object, for instance, the endTime property specifies the moment
    /// the activity concluded or is expected to conclude.
    ///
    /// - Range: `xsd:dateTime`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String, UtcTime), functional)]
    pub end_time: Option<serde_json::Value>,

    /// Identifies the entity (e.g. an application) that generated the object.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub generator: Option<serde_json::Value>,

    /// Indicates an entity that describes an icon for this object.
    ///
    /// The image should have an aspect ratio of one (horizontal) to one (vertical) and should be
    /// suitable for presentation at a small size.
    ///
    /// - Range: `Image` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link), concrete(Image))]
    pub icon: Option<serde_json::Value>,

    /// Indicates an entity that describes an image for this object.
    ///
    /// Unlike the icon property, there are no aspect ratio or display size limitations assumed.
    ///
    /// - Range: `Image` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link), concrete(Image))]
    pub image: Option<serde_json::Value>,

    /// Indicates one or more entities for which this object is considered a response.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub in_reply_to: Option<serde_json::Value>,

    /// Indicates one or more physical or logical locations associated with the object.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub location: Option<serde_json::Value>,

    /// Identifies an entity that provides a preview of this object.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub preview: Option<serde_json::Value>,

    /// The date and time at which the object was published.
    ///
    /// - Range: `xsd:dateTime`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String, UtcTime), functional)]
    pub published: Option<serde_json::Value>,

    /// Identifies a `Collection` containing objects considered to be responses to this object.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Collection), functional)]
    pub replies: Option<serde_json::Value>,

    /// The date and time describing the actual or expected starting time of the object.
    ///
    /// When used with an `Activity` object, for instance, the `start_time` property specifies the
    /// moment the activity began or is scheduled to begin.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String, UtcTime), functional)]
    pub start_time: Option<serde_json::Value>,

    // TODO: rdf:langString
    /// A natural language summarization of the object encoded as HTML.
    ///
    /// Multiple language tagged summaries MAY be provided.
    ///
    /// - Range: `xsd:string` | `rdf:langString`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String))]
    pub summary: Option<serde_json::Value>,

    /// One or more "tags" that have been associated with an objects. A tag can be any kind of
    /// `Object`.
    ///
    /// The key difference between attachment and tag is that the former implies association by
    /// inclusion, while the latter implies associated by reference.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub tag: Option<serde_json::Value>,

    /// The date and time at which the object was updated,
    ///
    /// - Range: `xsd:dateTime`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String, UtcTime), functional)]
    pub updated: Option<serde_json::Value>,

    /// Identifies one or more links to representations of the object.
    ///
    /// - Range: `xsd:anyUri` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), ab(Link))]
    pub url: Option<serde_json::Value>,

    /// Identifies an entity considered to be part of the public primary audience of an `Object`.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub to: Option<serde_json::Value>,

    /// Identifies an `Object` that is part of the private primary audience of this `Object`.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub bto: Option<serde_json::Value>,

    /// Identifies an `Object` that is part of the public secondary audience of this `Object`.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub cc: Option<serde_json::Value>,

    /// Identifies one or more `Objects` that are part of the private secondary audience of this
    /// `Object`.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object, Link))]
    pub bcc: Option<serde_json::Value>,

    /// When used on an `Object`, identifies the MIME media type of the value of the content
    /// property.
    ///
    /// If not specified, the content property is assumed to contain text/html content.
    ///
    /// - Range: `Mime Media Type`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    pub media_type: Option<serde_json::Value>,

    // TODO: xsd:duration
    /// When the object describes a time-bound resource, such as an audio or video, a meeting, etc,
    /// the duration property indicates the object's approximate duration.
    ///
    /// The value MUST be expressed as an xsd:duration as defined by
    /// [[xmlschema11-2](https://www.w3.org/TR/xmlschema11-2/)], section
    /// 3.3.6 (e.g. a period of 5 seconds is represented as "PT5S").
    ///
    /// - Range: `xsd:duration`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    pub duration: Option<serde_json::Value>,
}

impl ObjectProperties {
    /// Fetch a typed `Mime` struct from the `media_type` field.
    pub fn media_type(&self) -> Result<mime::Mime> {
        self.media_type_string()
            .and_then(|s| s.parse().map_err(|_| Error::Deserialize))
    }
}

/// Define all the properties of the Location type as described by the Activity Streams vocabulary.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct PlaceProperties {
    /// Indicates the accuracy of position coordinates on a `Place` objects.
    ///
    /// Expressed in properties of percentage. e.g. "94.0" means "94.0% accurate".
    ///
    /// - Range: `xsd:float` [>= 0.0f, <= 100.0f]
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    pub accuracy: Option<serde_json::Value>,

    /// Indicates the altitude of a place. The measurement units is indicated using the units
    /// property.
    ///
    /// If units is not specified, the default is assumed to be "m" indicating meters.
    ///
    /// - Range: `xsd:float`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    pub altitude: Option<serde_json::Value>,

    /// The latitude of a place.
    ///
    /// - Range: `xsd:float`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    pub latitude: Option<serde_json::Value>,

    /// The longitude of a place.
    ///
    /// - Range: `xsd:float`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    pub longitude: Option<serde_json::Value>,

    /// The radius from the given latitude and longitude for a Place.
    ///
    /// The units is expressed by the units property. If units is not specified, the default is
    /// assumed to be "m" indicating "meters".
    ///
    /// - Range: `xsd:float`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(f64), functional)]
    pub radius: Option<serde_json::Value>,

    /// Specifies the measurement units for the radius and altitude properties on a `Place` object.
    ///
    /// If not specified, the default is assumed to be "m" for "meters".
    ///
    /// - Range: `xsd:float`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String), functional)]
    pub units: Option<serde_json::Value>,
}

/// Define all the properties of the Profile type as described by the Activity Streams vocabulary.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct ProfileProperties {
    /// On a `Profile` object, the describes property identifies the object described by the
    /// `Profile`.
    ///
    /// - Range: `Object`
    /// - Functional: true
    #[activitystreams(ab(Object), functional)]
    pub describes: serde_json::Value,
}

/// Define all the properties of the Relationship type as described by the Activity Streams
/// vocabulary.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipProperties {
    /// On a `Relationship` object, the subject property identifies one of the connected
    /// individuals.
    ///
    /// For instance, for a `Relationship` object describing "John is related to Sally", subject
    /// would refer to John.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: true
    #[activitystreams(ab(Object, Link), functional)]
    subject: serde_json::Value,

    /// When used within a `Relationship` describes the entity to which the subject is related.
    ///
    /// - Range: `Object` | `Link`
    /// - Functional: false
    #[activitystreams(ab(Object, Link))]
    object: serde_json::Value,

    /// On a `Relationship` object, the relationship property identifies the kind of relationship
    /// that exists between subject and object.
    ///
    /// - Range: `Object`
    /// - Functional: false
    #[activitystreams(ab(Object))]
    relationship: serde_json::Value,
}

/// Define all the properties of the Tombstone type as described by the Activity Streams vocabulary.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct TombstoneProperties {
    /// On a `Tombstone` object, the formerType property identifies the type of the object that was
    /// deleted.
    ///
    /// - Range: `Object`
    /// - Functional: false
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(ab(Object))]
    pub former_type: Option<serde_json::Value>,

    /// On a `Tombstone` object, the deleted property is a timestamp for when the object was
    /// deleted.
    ///
    /// - Range: `xsd:dateTime`
    /// - Functional: true
    #[serde(skip_serializing_if = "Option::is_none")]
    #[activitystreams(concrete(String, UtcTime), functional)]
    pub deleted: Option<serde_json::Value>,
}
