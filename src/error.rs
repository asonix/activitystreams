use std::result;

#[derive(Copy, Clone, Debug, Fail)]
pub enum Error {
    #[fail(display = "Key not present")]
    NotFound,
    #[fail(display = "Failed to deserialize data as requested type")]
    Deserialize,
}

pub type Result<T> = result::Result<T, Error>;
