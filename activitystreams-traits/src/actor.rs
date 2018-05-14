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

/// `Actor` types are `Object` types that are capable of performing activities.
///
/// This specification intentionally defines `Actors` in only the most generalized way, stopping
/// short of defining semantically specific properties for each. All Actor objects are
/// specializations of `Object` and inherit all of the core properties common to all Objects.
/// External vocabularies can be used to express additional detail not covered by the Activity
/// Vocabulary. VCard [[vcard-rdf](https://www.w3.org/TR/vcard-rdf/) SHOULD be used to provide
/// additional metadata for `Person`, `Group`, and `Organization` instances.
///
/// While implementations are free to introduce new types of Actors beyond those defined by the
/// Activity Vocabulary, interoperability issues can arise when applications rely too much on
/// extension types that are not recognized by other implementations. Care should be taken to not
/// unduly overlap with or duplicate the existing `Actor` types.
///
/// When an implementation uses an extension type that overlaps with a core vocabulary type, the
/// implementation MUST also specify the core vocabulary type. For instance, some vocabularies
/// (e.g. VCard) define their own types for describing people. An implementation that wishes, for
/// example, to use a `vcard:Individual` as an `Actor` MUST also identify that `Actor` as a
/// `Person`.
pub trait Actor: Object {}
