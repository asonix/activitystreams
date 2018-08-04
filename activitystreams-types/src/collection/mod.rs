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

//! Namespace for Collection types

use activitystreams_traits::{Collection, CollectionPage, Object};

use object::{properties::ObjectProperties, ObjectExt};

pub mod kind;
pub mod properties;
use self::kind::*;
use self::properties::*;

/// The Collection Extension Trait
///
/// This trait provides generic access to a collection's properties
pub trait CollectionExt: Collection {
    fn props(&self) -> &CollectionProperties;
    fn props_mut(&mut self) -> &mut CollectionProperties;
}

/// The Collection Page Extension Trait
///
/// This trait provides generic access to a collection page's properties
pub trait CollectionPageExt: CollectionPage {
    fn props(&self) -> &CollectionPageProperties;
    fn props_mut(&mut self) -> &mut CollectionPageProperties;
}

/// The default `Collection` type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct UnorderedCollection {
    #[serde(rename = "type")]
    kind: CollectionType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    pub collection_props: CollectionProperties,
}

impl Object for UnorderedCollection {}
impl ObjectExt for UnorderedCollection {
    fn props(&self) -> &ObjectProperties {
        &self.object_props
    }

    fn props_mut(&mut self) -> &mut ObjectProperties {
        &mut self.object_props
    }
}
impl Collection for UnorderedCollection {}
impl CollectionExt for UnorderedCollection {
    fn props(&self) -> &CollectionProperties {
        &self.collection_props
    }

    fn props_mut(&mut self) -> &mut CollectionProperties {
        &mut self.collection_props
    }
}

/// A subtype of `Collection` in which members of the logical collection are assumed to always be
/// strictly ordered.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(rename = "type")]
    kind: OrderedCollectionType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    pub collection_props: CollectionProperties,
}

impl Object for OrderedCollection {}
impl ObjectExt for OrderedCollection {
    fn props(&self) -> &ObjectProperties {
        &self.object_props
    }

    fn props_mut(&mut self) -> &mut ObjectProperties {
        &mut self.object_props
    }
}
impl Collection for OrderedCollection {}
impl CollectionExt for OrderedCollection {
    fn props(&self) -> &CollectionProperties {
        &self.collection_props
    }

    fn props_mut(&mut self) -> &mut CollectionProperties {
        &mut self.collection_props
    }
}

/// Used to represent distinct subsets of items from a `Collection`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct UnorderedCollectionPage {
    #[serde(rename = "type")]
    kind: CollectionPageType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    pub collection_props: CollectionProperties,

    /// Adds all valid collection page properties to this struct
    #[serde(flatten)]
    pub collection_page_props: CollectionPageProperties,
}

impl Object for UnorderedCollectionPage {}
impl ObjectExt for UnorderedCollectionPage {
    fn props(&self) -> &ObjectProperties {
        &self.object_props
    }

    fn props_mut(&mut self) -> &mut ObjectProperties {
        &mut self.object_props
    }
}
impl Collection for UnorderedCollectionPage {}
impl CollectionExt for UnorderedCollectionPage {
    fn props(&self) -> &CollectionProperties {
        &self.collection_props
    }

    fn props_mut(&mut self) -> &mut CollectionProperties {
        &mut self.collection_props
    }
}
impl CollectionPage for UnorderedCollectionPage {}
impl CollectionPageExt for UnorderedCollectionPage {
    fn props(&self) -> &CollectionPageProperties {
        &self.collection_page_props
    }

    fn props_mut(&mut self) -> &mut CollectionPageProperties {
        &mut self.collection_page_props
    }
}

/// Used to represent ordered subsets of items from an `OrderedCollection`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollectionPage {
    #[serde(rename = "type")]
    kind: OrderedCollectionPageType,

    /// Adds all valid object properties to this struct
    #[serde(flatten)]
    pub object_props: ObjectProperties,

    /// Adds all valid collection properties to this struct
    #[serde(flatten)]
    pub collection_props: CollectionProperties,

    /// Adds all valid collection page properties to this struct
    #[serde(flatten)]
    pub collection_page_props: CollectionPageProperties,

    /// Adds all valid ordered collection page properties to this struct
    #[serde(flatten)]
    pub ordered_collection_page_props: OrderedCollectionPageProperties,
}

impl Object for OrderedCollectionPage {}
impl ObjectExt for OrderedCollectionPage {
    fn props(&self) -> &ObjectProperties {
        &self.object_props
    }

    fn props_mut(&mut self) -> &mut ObjectProperties {
        &mut self.object_props
    }
}
impl Collection for OrderedCollectionPage {}
impl CollectionExt for OrderedCollectionPage {
    fn props(&self) -> &CollectionProperties {
        &self.collection_props
    }

    fn props_mut(&mut self) -> &mut CollectionProperties {
        &mut self.collection_props
    }
}
impl CollectionPage for OrderedCollectionPage {}
impl CollectionPageExt for OrderedCollectionPage {
    fn props(&self) -> &CollectionPageProperties {
        &self.collection_page_props
    }

    fn props_mut(&mut self) -> &mut CollectionPageProperties {
        &mut self.collection_page_props
    }
}
