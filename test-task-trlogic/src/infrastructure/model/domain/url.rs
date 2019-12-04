use crate::domain::model::url::Url;
use std::str::FromStr;


use serde::{Deserialize, Deserializer};
use serde::de::{Error as SerdeError};
use serde::{Serialize, Serializer};

impl Serialize for Url {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_str(self.as_str())
    }
}

impl <'de>Deserialize<'de> for Url {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let url = String::deserialize(deserializer)?;
        let url = Url::from_str(&url).map_err(D::Error::custom)?;
        return Ok(url);
    }
}