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

/// An Activity is a subtype of `Object` that describes some form of action that may happen, is
/// currently happening, or has already happened.
///
/// The `Activity` type itself serves as an abstract base type for all types of activities. It is
/// important to note that the `Activity` type itself does not carry any specific semantics about
/// the kind of action being taken.
pub trait Activity: Object {}

/// Instances of `IntransitiveActivity` are a subtype of `Activity` representing intransitive
/// actions.
///
/// The `object` property is therefore inappropriate for these activities.
pub trait IntransitiveActivity: Activity {}
