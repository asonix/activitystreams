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

#[macro_use]
extern crate activitystreams_derive;
extern crate chrono;
#[macro_use]
extern crate failure;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub fn context() -> serde_json::Value {
    json!({
        "one": "two",
    })
}

pub trait Properties {
    fn get_value<F, I>(&self, f: F) -> error::Result<I>
    where
        F: FnOnce(&Self) -> &serde_json::Value,
        I: serde::de::DeserializeOwned,
    {
        serde_json::from_value(f(self).clone()).map_err(|_| error::Error::Deserialize)
    }

    fn get_item<F, I>(&self, f: F) -> error::Result<I>
    where
        F: FnOnce(&Self) -> &Option<serde_json::Value>,
        I: serde::de::DeserializeOwned,
    {
        if let &Some(ref item) = f(self) {
            serde_json::from_value(item.clone()).map_err(|_| error::Error::Deserialize)
        } else {
            Err(error::Error::NotFound)
        }
    }

    fn get_vec<F, I>(&self, f: F) -> error::Result<Vec<I>>
    where
        F: FnOnce(&Self) -> &Vec<serde_json::Value>,
        I: serde::de::DeserializeOwned,
    {
        let item = f(self);

        item.iter().fold(Ok(Vec::new()), |acc, item| match acc {
            Ok(mut acc) => match serde_json::from_value(item.clone()) {
                Ok(item) => {
                    acc.push(item);
                    Ok(acc)
                }
                Err(_) => Err(error::Error::Deserialize),
            },
            Err(e) => Err(e),
        })
    }
}

pub mod activity;
pub mod actor;
pub mod collection;
pub mod error;
pub mod link;
pub mod object;
