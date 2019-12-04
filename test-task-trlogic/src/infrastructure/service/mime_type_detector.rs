use crate::domain::service::mime_type_detector::MimeTypeDetectorService;
use crate::domain::model::file::File;
use crate::domain::model::mime_type::MimeType;
use failure::Error;

#[derive(new)]
pub struct MimeTypeDetector {

}

impl MimeTypeDetectorService for MimeTypeDetector {
    fn detect_mime_type(&self, file: &File) -> Result<MimeType, Error> {
        let filename_extension = match file.get_filename_ext() {
            Some(filename_extension) => filename_extension,
            None => {
                return Ok(MimeType::new_application_octet_stream());
            },
        };
        let mime_type = mime_guess::from_ext(filename_extension);
        let mime_type: MimeType = mime_type.first_or_octet_stream().into();
        return Ok(mime_type);
    }
}