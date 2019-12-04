use crate::domain::model::url::Url;
use crate::domain::model::file_id::FileId;
use failure::Error;
pub trait UrlGeneratorService {
    fn generate_fetch_file_url(&self, file_id: &FileId) -> Result<Url, Error>;
    fn generate_image_preview_url(&self, file_id: &FileId) -> Result<Url, Error>;
}