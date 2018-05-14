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

use serde::{de::DeserializeOwned, ser::Serialize};

/// A Link is an indirect, qualified reference to a resource identified by a URL.
///
/// The fundamental model for links is established by
/// [[RFC5988](https://tools.ietf.org/html/rfc5988)]. Many of the properties defined by the
/// Activity Vocabulary allow values that are either instances of Object or Link. When a Link is
/// used, it establishes a qualified relation connecting the subject (the containing object) to the
/// resource identified by the href. Properties of the Link are properties of the reference as
/// opposed to properties of the resource.
pub trait Link: DeserializeOwned + Serialize {}
