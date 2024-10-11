mod parameters;
pub use parameters::LweParameters;

mod server_key;
pub use server_key::ServerKey;

mod integer;
pub use integer::{FheElement, FheUInt8};

#[cfg(test)]
mod tests;

use std::io::{Read, Write};
use std::path::Path;
use std::{fs::File, io::Cursor};

use std::error;

use winterfell::{Deserializable, Serializable};

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
    Self: Serializable,
{
    fn export_to_file(&self, path: &Path) -> Result<(), Error> {
        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(Error::new(err.to_string().to_lowercase())),
        };

        let mut content = Vec::new();

        self.write_into(&mut content);

        if let Err(err) = file.write_all(&content) {
            return Err(Error::new(err.to_string().to_lowercase()));
        }
        Ok(())
    }
}

pub trait Import
where
    Self: Deserializable,
{
    fn import_from_file(path: &Path) -> Result<Self, Error> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Err(Error::new(err.to_string().to_lowercase())),
        };

        let mut buffer = Vec::new();

        match file.read_to_end(&mut buffer) {
            Ok(_) => (),
            Err(err) => return Err(Error::new(err.to_string().to_lowercase())),
        };

        let mut cursor = Cursor::new(buffer);

        match Self::read_from(&mut cursor) {
            Ok(value) => Ok(value),
            Err(err) => Err(Error::new(err.to_string().to_lowercase())),
        }
    }
}
