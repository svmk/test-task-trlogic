use crate::domain::service::filename_generator::FileNameGeneratorService;
use crate::domain::model::mime_type::MimeType;
use mime_guess::get_mime_extensions;
use failure::{Error, err_msg};

#[derive(new)]
pub struct FileNameGenerator {

}

impl FileNameGeneratorService for FileNameGenerator {
    fn generate_filename(&self, mime_type: &MimeType) -> Result<String, Error> {
        let extensions = get_mime_extensions(mime_type.into()).unwrap_or(&[]);
        if let Some(extension) = extensions.get(0) {
            let extension = *extension;
            return Ok(extension.into());
        }
        return Err(err_msg(format!("Unable to find extension for mime type {}", mime_type)));
    }
}
