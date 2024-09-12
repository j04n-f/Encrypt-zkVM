mod parameters;
pub use parameters::LweParameters;

mod server_key;
pub use server_key::ServerKey;

mod integer;
pub use integer::FheUInt8;

#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::de::DeserializeOwned;
use serde::Serialize;

use std::error;

pub struct Error {
    message: String,
}

impl error::Error for Error {}

impl Error {
    pub fn new(message: String) -> Error {
        Error { message }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub trait Export
where
    Self: Serialize,
{
    fn export_to_file(&self, path: &Path) -> Result<(), Error> {
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(Error::new(err.to_string().to_lowercase())),
        };
        let mut content = Vec::new();
        if let Err(err) = bincode::serialize_into(&mut content, &self) {
            return Err(Error::new(err.to_string().to_lowercase()));
        }
        if let Err(err) = file.write_all(&content) {
            return Err(Error::new(err.to_string().to_lowercase()));
        }
        Ok(())
    }
}

pub trait Import
where
    Self: DeserializeOwned,
{
    fn import_from_file(path: &Path) -> Result<Self, Error> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(Error::new(err.to_string().to_lowercase())),
        };
        match bincode::deserialize_from(&mut file) {
            Ok(value) => Ok(value),
            Err(err) => Err(Error::new(err.to_string().to_lowercase())),
        }
    }
}
