use serde::{de::DeserializeOwned, ser::Serialize};

pub trait Base: Serialize + DeserializeOwned {}
