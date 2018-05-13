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

use serde::{de::DeserializeOwned, ser::Serialize};

pub mod kind;
pub mod properties;
use self::kind::*;
use self::properties::*;

/// The Object is the primary base type for the Activity Streams vocabulary.
pub trait Object: DeserializeOwned + Serialize {}

/// Represents any kind of multi-paragraph written work.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    #[serde(rename = "type")]
    kind: ArticleType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Article {}

/// Represents an audio document of any kind.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Audio {
    #[serde(rename = "type")]
    kind: AudioType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Audio {}

/// Represents a document of any kind.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    #[serde(rename = "type")]
    kind: DocumentType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Document {}

/// Represents any kind of event.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "type")]
    kind: EventType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Event {}

/// An image document of any kind
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    kind: ImageType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Image {}

/// Represents a short written work typically less than a single paragraph in length.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(rename = "type")]
    kind: NoteType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Note {}

/// Represents a Web Page.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    #[serde(rename = "type")]
    kind: PageType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Page {}

/// Represents a logical or physical location.
///
/// The Place object is used to represent both physical and logical locations. While numerous
/// existing vocabularies exist for describing locations in a variety of ways, inconsistencies and
/// incompatibilities between those vocabularies make it difficult to achieve appropriate
/// interoperability between implementations. The Place object is included within the Activity
/// vocabulary to provide a minimal, interoperable starting point for describing locations
/// consistently across Activity Streams 2.0 implementations.
///
/// The Place object is intentionally flexible. It can, for instance, be used to identify a location
/// simply by name, or by longitude and latitude.
///
/// The Place object can also describe an area around a given point using the radius property, the
/// altitude of the location, and a degree of accuracy.
///
/// While publishers are not required to use these specific properties and MAY make use of other
/// mechanisms for describing locations, consuming implementations that support the Place object
/// MUST support the use of these properties.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Place {
    #[serde(rename = "type")]
    kind: PlaceType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid place properties to this struct
    #[serde(flatten)]
    pub place: PlaceProperties,
}

impl Object for Place {}

/// A Profile is a content object that describes another `Object`, typically used to describe
/// `Actor` Type objects.
///
/// The `describes` property is used to reference the object being described by the profile.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(rename = "type")]
    kind: ProfileType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid profile properties to this struct
    #[serde(flatten)]
    pub profile: ProfileProperties,
}

impl Object for Profile {}

/// Describes a relationship between two individuals.
///
/// The subject and object properties are used to identify the connected individuals.
///
/// The `Relationship` object is used to represent relationships between individuals. It can be
/// used, for instance, to describe that one person is a friend of another, or that one person is a
/// member of a particular organization. The intent of modeling Relationship in this way is to allow
/// descriptions of activities that operate on the relationships in general, and to allow
/// representation of Collections of relationships.
///
/// For instance, many social systems have a notion of a "friends list". These are the collection of
/// individuals that are directly connected within a person's social graph. Suppose we have a user,
/// Sally, with direct relationships to users Joe and Jane. Sally follows Joe's updates while Sally
/// and Jane have a mutual relationship.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    #[serde(rename = "type")]
    kind: RelationshipType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid relationship properties to this struct
    #[serde(flatten)]
    pub relationship: RelationshipProperties,
}

impl Object for Relationship {}

/// A Tombstone represents a content object that has been deleted.
///
/// It can be used in Collections to signify that there used to be an object at this position, but
/// it has been deleted.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tombstone {
    #[serde(rename = "type")]
    kind: TombstoneType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid tombstone properties to this struct
    #[serde(flatten)]
    pub tombstone_props: TombstoneProperties,
}

impl Object for Tombstone {}

/// Represents a video document of any kind.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    #[serde(rename = "type")]
    kind: VideoType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,
}

impl Object for Video {}
