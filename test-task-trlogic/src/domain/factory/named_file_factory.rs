use crate::domain::model::downloaded_file::DownloadedFile;
use crate::domain::model::file::File;
use crate::domain::model::uploaded_file::UploadedFile;
use failure::Error;

pub trait NamedFileFactory {
    fn from_downloaded_file(&self, downloaded_file: DownloadedFile) -> Result<File, Error>;
    fn from_uploaded_file(&self, uploaded_file: UploadedFile) -> Result<File, Error>;
}