use crate::presentation::web::request::upload_image_request::UploadImageRequest;
use crate::presentation::web::error::ValidationError;
use crate::presentation::web::response::upload_image_response::{UploadImageResponse, UploadImageResponseItem};
use crate::domain::service::file_downloading::FileDownloadingService;
use crate::domain::factory::named_file_factory::NamedFileFactory;
use crate::domain::repository::file_repository::FileRepository;
use crate::domain::di::service_ref::ServiceRef;
use crate::domain::service::url_generator::UrlGeneratorService;
use crate::domain::model::downloaded_file::DownloadedFile;
use crate::presentation::web::error::WebError;


#[derive(new)]
pub struct UploadImageController {
    file_downloading: ServiceRef<&'static dyn FileDownloadingService>,
    named_file_factory: ServiceRef<&'static dyn NamedFileFactory>,
    file_repository: ServiceRef<&'static dyn FileRepository>,
    url_generator: ServiceRef<&'static dyn UrlGeneratorService>,
}

impl UploadImageController {
    pub async fn upload_image(&self, body: UploadImageRequest) -> Result<UploadImageResponse, WebError> {
        self.validate_request(&body)?;
        let mut uploaded_files = Vec::new();
        for file_external_url in body.iter_urls() {
            let downloaded_file = self.file_downloading.download_file(file_external_url).await?;
            self.validate_external_file(&downloaded_file)?;
            let file = self.named_file_factory.from_downloaded_file(downloaded_file)?;
            uploaded_files.push(file);
        }
        for uploaded_file in body.into_uploaded_files().drain(..) {
            let file = self.named_file_factory.from_uploaded_file(uploaded_file)?;
            uploaded_files.push(file);
        }
        let mut result = UploadImageResponse::new();
        for file in uploaded_files.drain(..) {
            let file = self.file_repository.add(file)?;
            let image_preview_url = self.url_generator.generate_image_preview_url(file.get_id())?;
            let file_url = self.url_generator.generate_fetch_file_url(file.get_id())?;
            let item = UploadImageResponseItem::new(
                file.get_id().clone(),
                file_url,
                image_preview_url,
            );
            result.add_item(item);
        }
        return Ok(result);
    }

    fn validate_request(&self, body: &UploadImageRequest) -> Result<(), ValidationError> {
        if !body.has_urls() && !body.has_uploaded_files() {
            return Err(ValidationError::new("There is no uploaded files or urls"));
        }
        for (file_no, file) in body.iter_uploaded_files().enumerate() {
            if !file.get_mime_type().is_image() {
                let error = ValidationError::new(format!("Uploaded file no {} with mime type {} is not image.", file_no, file.get_mime_type()));
                return Err(error);
            }
        }
        return Ok(());
    }

    fn validate_external_file(&self, downloaded_file: &DownloadedFile) -> Result<(), ValidationError> {
        if !downloaded_file.get_mime_type().is_image() {
            let error = ValidationError::new(format!("Fetched file at url `{}` must contain image", downloaded_file.get_fetched_url()));
            return Err(error);
        }
        return Ok(());
    }
}