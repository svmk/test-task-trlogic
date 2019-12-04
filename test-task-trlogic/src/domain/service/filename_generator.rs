use failure::Error;
use crate::domain::model::mime_type::MimeType;

pub trait FileNameGeneratorService {
    fn generate_filename(&self, mime_type: &MimeType) -> Result<String, Error>;
}