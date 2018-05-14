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

//! A module containing helpers for tranlsating common JSON representations to and from concrete
//! types

use serde::{de::DeserializeOwned, ser::Serialize};
use serde_json;

use error::{Error, Result};

/// Deserialize a `Value` into concrete type I
pub fn from_value<I>(item: &serde_json::Value) -> Result<I>
where
    I: DeserializeOwned,
{
    serde_json::from_value(item.clone()).map_err(|_| Error::Deserialize)
}

/// Serialize concrete type I into a `Value`
pub fn to_value<I>(item: I) -> Result<serde_json::Value>
where
    I: Serialize,
{
    serde_json::to_value(item).map_err(|_| Error::Serialize)
}

/// Deserialize an `Option<Value>` into concrete type I
pub fn from_item<I>(item: &Option<serde_json::Value>) -> Result<I>
where
    I: DeserializeOwned,
{
    if let &Some(ref item) = item {
        from_value(item)
    } else {
        Err(Error::NotFound)
    }
}

/// Serialize concrete type I into an `Option<Value>`
pub fn to_item<I>(item: I) -> Result<Option<serde_json::Value>>
where
    I: Serialize,
{
    to_value(item).map(Some)
}

/// Deserialize a `Vec<Value>` into a `Vec<I>`
pub fn from_vec<I>(v: &Vec<serde_json::Value>) -> Result<Vec<I>>
where
    I: DeserializeOwned,
{
    v.iter().fold(Ok(Vec::new()), |acc, item| match acc {
        Ok(mut acc) => from_value(item).map(|item| {
            acc.push(item);
            acc
        }),
        e => e,
    })
}

/// Serialize a `Vec<I>` into a `Vec<Value>`
pub fn to_vec<I>(v: Vec<I>) -> Result<Vec<serde_json::Value>>
where
    I: Serialize,
{
    v.into_iter().fold(Ok(Vec::new()), |acc, item| match acc {
        Ok(mut acc) => to_value(item).map(|item| {
            acc.push(item);
            acc
        }),
        e => e,
    })
}
