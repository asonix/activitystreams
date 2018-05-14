/*
 * This file is part of ActivityStreams Traits.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityStreams Traits is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Traits is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Traits.  If not, see <http://www.gnu.org/licenses/>.
 */

use object::Object;

/// A Collection is a subtype of `Object` that represents ordered or unordered sets of `Object` or
/// `Link` instances.
///
/// The items within a Collection can be ordered or unordered. The OrderedCollection type MAY be
/// used to identify a Collection whose items are always ordered. In the JSON serialization, the
/// unordered items of a Collection are represented using the items property while ordered items
/// are represented using the orderedItems property.
///
/// `UnorderedCollection` and `OrderedCollection` types are provided by the `activitystreams-types`
/// crate.
pub trait Collection: Object {}

/// Used to represent distinct subsets of items from a Collection.
///
/// A `Collection` can contain a large number of items. Often, it becomes impractical for an
/// implementation to serialize every item contained by a `Collection` using the items (or
/// `ordered_items`) property alone. In such cases, the items within a `Collection` can be divided
/// into distinct subsets or "pages". A page is identified using the `CollectionPage` type.
///
/// `UnorderedCollectionPage` and `OrderedCollectionPage` types are provied by the
/// `activitystreams-types` crate.
pub trait CollectionPage: Collection {}
