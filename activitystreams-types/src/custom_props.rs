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

//! A collection of simple types for extending the ActivityStreams Types base types

use serde::{de::DeserializeOwned, ser::Serialize};

use activitystreams_traits::{
    Activity, Actor, Collection, CollectionPage, IntransitiveActivity, Link, Object,
};

/// A custom type extending Link
///
/// CustomLink allows for providing a pre-defined Link type, and a set of extending properties, and
/// treating those two items as a single Link type.
///
/// ## Example
/// ```rust
/// use activitystreams_types::{
///     CustomLink,
///     link::Mention,
/// };
///
/// struct MyProps {
///     some_prop: String,
/// }
///
/// fn main() {
///     let mention = Mention::default();
///     let extended_mention = CustomLink::new(mention, MyProps { some_prop: "hey".to_owned() });
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomLink<C, L> {
    #[serde(flatten)]
    pub link: L,

    #[serde(flatten)]
    pub custom_props: C,
}

impl<C, L: Link> CustomLink<C, L> {
    pub fn new(link: L, custom_props: C) -> Self {
        CustomLink { link, custom_props }
    }
}

impl<C, L> Link for CustomLink<C, L>
where
    C: DeserializeOwned + Serialize,
    L: Link,
{
}

/// A custom type extending Object
///
/// CustomObject allows for providing a pre-defined Link type, and a set of extending properties,
/// and treating those two items as a single Object type.
///
/// This type can also be used to extend any type deriving from Object, such as Actor, Activity, or
/// Collection.
///
/// ## Example
/// ```rust
/// use activitystreams_types::{
///     CustomObject,
///     object::Video,
/// };
///
/// struct MyProps {
///     some_prop: String,
/// }
///
/// fn main() {
///     let video = Video::default();
///     let extended_video = CustomObject::new(video, MyProps { some_prop: "hey".to_owned() });
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomObject<C, O> {
    #[serde(flatten)]
    pub object: O,

    #[serde(flatten)]
    pub custom_props: C,
}

impl<C, O: Object> CustomObject<C, O> {
    pub fn new(object: O, custom_props: C) -> Self {
        CustomObject {
            object,
            custom_props,
        }
    }
}

impl<C, O> Object for CustomObject<C, O>
where
    C: DeserializeOwned + Serialize,
    O: Object,
{
}
impl<C, O> Actor for CustomObject<C, O>
where
    C: DeserializeOwned + Serialize,
    O: Actor,
{
}
impl<C, O> Collection for CustomObject<C, O>
where
    C: DeserializeOwned + Serialize,
    O: Collection,
{}
impl<C, O> CollectionPage for CustomObject<C, O>
where
    C: DeserializeOwned + Serialize,
    O: CollectionPage,
{}
impl<C, O> Activity for CustomObject<C, O>
where
    C: DeserializeOwned + Serialize,
    O: Activity,
{}
impl<C, O> IntransitiveActivity for CustomObject<C, O>
where
    C: DeserializeOwned + Serialize,
    O: IntransitiveActivity,
{}
