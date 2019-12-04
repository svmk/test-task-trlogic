use crate::domain::service::url_generator::UrlGeneratorService;
use failure::Error;
use crate::domain::model::url::Url;
use crate::domain::model::file_id::FileId;

#[derive(new)]
pub struct UrlGenerator {
    base_url: Url,
}

impl UrlGeneratorService for UrlGenerator {
    fn generate_fetch_file_url(&self, file_id: &FileId) -> Result<Url, Error> {
        let url = self
            .base_url
            .clone()
            .join_path("api")?
            .join_path("v1")?
            .join_path("files")?
            .join_path(file_id.to_string())?;
        return Ok(url);
    }

    fn generate_image_preview_url(&self, file_id: &FileId) -> Result<Url, Error> {
        let url = self.generate_fetch_file_url(file_id)?;
        let url = url.join_path("image-preview")?;
        return Ok(url);
    }
}