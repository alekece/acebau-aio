use serde::{de::DeserializeOwned, Serialize};

use crate::{format::Serializable, types::id::Generate};

pub trait Entity: Serialize + DeserializeOwned {
    type Id: Generate + Clone + PartialEq + Eq;
    type Format: Serializable<Self>;

    fn schema_name() -> &'static str;
}
