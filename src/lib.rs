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
}

pub mod activity;
pub mod actor;
pub mod base;
pub mod collection;
pub mod error;
pub mod link;
pub mod object;
