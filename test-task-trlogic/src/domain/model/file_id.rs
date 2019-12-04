use uuid::Uuid;
use std::str::FromStr;
use failure::{Error, err_msg};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FileId(Uuid);

impl FileId {
    pub fn new() -> FileId {
        return FileId(Uuid::new_v4());
    }

    pub fn to_string(&self) -> String {
        return format!("{}", self.0);
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl FromStr for FileId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::from_str(s).map_err(err_msg)?;
        return Ok(FileId(uuid));
    }
}