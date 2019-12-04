use failure::Error;
use crate::domain::model::mime_type::MimeType;
use crate::domain::model::file::File;
pub trait MimeTypeDetectorService {
    fn detect_mime_type(&self, file: &File) -> Result<MimeType, Error>;
}