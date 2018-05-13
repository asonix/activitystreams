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

use link::Link;
use object::Object;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomLink<C, L> {
    #[serde(flatten)]
    link: L,

    #[serde(flatten)]
    pub custom_props: C,
}

impl<C, L: Link> CustomLink<C, L> {
    pub fn new(link: L, custom_props: C) -> Self {
        CustomLink { link, custom_props }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomObject<C, O> {
    #[serde(flatten)]
    object: O,

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
