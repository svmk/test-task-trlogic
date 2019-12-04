use crate::domain::model::temp_file_path::TempFilePath;
use tempfile::{TempPath, NamedTempFile};
use std::convert::From;
use failure::{Error, err_msg};

impl TempFilePath {
    pub fn new() -> Result<TempFilePath, Error> {
        let file = NamedTempFile::new().map_err(err_msg)?;
        let (_, path) = file.into_parts();
        let path = path.keep().map_err(err_msg)?;
        return Ok(TempFilePath::from_path(path));
    }

    pub fn new_with_filename(filename: &str) -> Result<TempFilePath, Error> {
        let dir = tempfile::tempdir()?;
        let dir = dir.into_path();
        let path = dir.join(filename);
        return Ok(TempFilePath::from_path(path));
    }
}

impl From<TempPath> for TempFilePath {
    fn from(file: TempPath) -> Self {
        return TempFilePath::from_path(file.to_path_buf());
    }
}

impl From<NamedTempFile> for TempFilePath {
    fn from(file: NamedTempFile) -> Self {
        let (_, file) = file.into_parts();
        return TempFilePath::from_path(file.to_path_buf());
    }
}