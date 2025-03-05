use serde::{de::DeserializeOwned, Serialize};

use crate::Result;

pub trait Serializable<T: Serialize + DeserializeOwned> {
    fn serialize(data: &T) -> Result<String>;
    fn deserialize(data: &str) -> Result<T>;
}

#[cfg(feature = "json")]
pub struct Json;

#[cfg(feature = "json")]
mod json {
    use crate::Error;

    use super::*;

    impl<T: Serialize + DeserializeOwned> Serializable<T> for Json {
        fn serialize(data: &T) -> Result<String> {
            serde_json::to_string(data).map_err(|e| Error::SerializeError(Box::new(e)))
        }

        fn deserialize(data: &str) -> Result<T> {
            serde_json::from_str(data).map_err(|e| Error::DeserializeError(Box::new(e)))
        }
    }
}

#[cfg(feature = "bson")]
pub struct Bson;

#[cfg(feature = "bson")]
mod bson {
    use ::bson::{Bson, Document};

    use crate::Error;
    use super::*;

    impl<T: Serialize + DeserializeOwned> Serializable<T> for Bson {
        fn serialize(data: &T) -> Result<String> {
            Ok(::bson::to_document(data)?.to_string())
        }

        fn deserialize(data: &str) -> Result<T> {
            Ok(::bson::from_document(Document::from_reader(&mut data.as_bytes())?)?)
        }
    }

    impl From<::bson::de::Error> for Error {
        fn from(e: ::bson::de::Error) -> Self {
            Error::DeserializeError(Box::new(e))
        }
    }

    impl From<::bson::ser::Error> for Error {
        fn from(e: ::bson::ser::Error) -> Self {
            Error::SerializeError(Box::new(e))
        }
    }
}
