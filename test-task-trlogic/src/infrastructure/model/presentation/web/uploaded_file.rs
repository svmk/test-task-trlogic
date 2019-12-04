use crate::domain::model::temp_file_path::TempFilePath;
use crate::domain::model::data_uri::DataUri;
use crate::domain::model::uploaded_file::UploadedFile;
use serde::{Deserialize, Deserializer};
use serde::de::Error;
use std::io::Write;
use std::str::FromStr;
use std::fs::File;

impl <'de>Deserialize<'de> for UploadedFile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        let data = String::deserialize(deserializer)?;
        let file_data = DataUri::from_str(&data).map_err(Error::custom)?;
        let mime_type = file_data.get_mime_type().clone();
        let tempfile = TempFilePath::new().map_err(Error::custom)?;
        let mut file = File::create(tempfile.get_path()).map_err(Error::custom)?;
        file.write_all(file_data.get_data()).map_err(Error::custom)?;
        let uploaded_file = UploadedFile::new(tempfile, mime_type);
        return Ok(uploaded_file);
    }
}