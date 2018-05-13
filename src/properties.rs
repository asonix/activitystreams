use serde::de::DeserializeOwned;
use serde_json;

use error::{Error, Result};

pub fn get_value<I>(item: &serde_json::Value) -> Result<I>
where
    I: DeserializeOwned,
{
    serde_json::from_value(item.clone()).map_err(|_| Error::Deserialize)
}

pub fn get_item<I>(item: &Option<serde_json::Value>) -> Result<I>
where
    I: DeserializeOwned,
{
    if let &Some(ref item) = item {
        serde_json::from_value(item.clone()).map_err(|_| Error::Deserialize)
    } else {
        Err(Error::NotFound)
    }
}

pub fn get_vec<I>(v: &Vec<serde_json::Value>) -> Result<Vec<I>>
where
    I: DeserializeOwned,
{
    v.iter().fold(Ok(Vec::new()), |acc, item| match acc {
        Ok(mut acc) => match serde_json::from_value(item.clone()) {
            Ok(item) => {
                acc.push(item);
                Ok(acc)
            }
            Err(_) => Err(Error::Deserialize),
        },
        Err(e) => Err(e),
    })
}
