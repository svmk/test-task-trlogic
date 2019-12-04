use crate::domain::model::file::File;
use crate::domain::model::file_id::FileId;
use failure::Error;

pub trait FileRepository {
    fn get(&self, file_id: &FileId) -> Result<Option<File>, Error>;
    fn add(&self, file: File) -> Result<File, Error>;
}