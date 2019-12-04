use crate::domain::model::data_uri::DataUri;
use serde::{Deserialize, Deserializer};
use serde::de::{Error as SerdeError};
use std::str::FromStr;
use failure::{Error, err_msg};
use mime::Mime;

impl FromStr for DataUri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data_url = data_url::DataUrl::process(s)
            .map_err(|error| {
                err_msg(format!("{:?}", error))
            })?;
        let mime_type = format!("{}", data_url.mime_type());
        let mime_type= Mime::from_str(&mime_type).map_err(err_msg)?;
        let (data, _) = data_url.decode_to_vec().map_err(|_| err_msg("Unable to parse data url"))?;
        return Ok(DataUri::new(mime_type.into(), data));
    }
}

impl <'de>Deserialize<'de> for DataUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let value = String::deserialize(deserializer)?;
        let value = Self::from_str(&value).map_err(D::Error::custom)?;
        return Ok(value);
    }
}