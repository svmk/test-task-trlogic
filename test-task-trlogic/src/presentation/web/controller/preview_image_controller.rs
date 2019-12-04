use crate::presentation::web::error::WebError;
use crate::domain::repository::file_repository::FileRepository;
use crate::domain::model::application_config::ImageConverter;
use crate::domain::di::service_ref::ServiceRef;
use crate::domain::model::file_id::FileId;
use crate::domain::service::mime_type_detector::MimeTypeDetectorService;
use crate::domain::service::image_converter::ImageConverterService;
use crate::presentation::web::response::file_response::FileResponse;

#[derive(new)]
pub struct PreviewImageController {
    image_converter_config: ImageConverter,
    file_repository: ServiceRef<&'static dyn FileRepository>,
    mime_type_detector: ServiceRef<&'static dyn MimeTypeDetectorService>,
    image_converter: ServiceRef<&'static dyn ImageConverterService>,
}

impl PreviewImageController {
    pub fn preview_image(&self, file_id: FileId) -> Result<Option<FileResponse>, WebError> {
        let file_info = self.file_repository.get(&file_id)?;
        let file_info = match file_info {
            Some(file_info) => file_info,
            None => {
                return Ok(None);
            }
        };
        let mime_type = self.mime_type_detector.detect_mime_type(&file_info)?;
        if !mime_type.is_image() {
            return Ok(None);
        }
        let image_file = self
            .image_converter
            .convert_image(&file_info, self.image_converter_config.get_preview())?;
        let response = FileResponse::inline(image_file);
        return Ok(Some(response));
    }
}