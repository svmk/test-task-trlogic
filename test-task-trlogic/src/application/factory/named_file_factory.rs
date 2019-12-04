use crate::domain::factory::named_file_factory::NamedFileFactory as NamedFileFactoryTrait;
use crate::domain::service::filename_generator::FileNameGeneratorService;
use crate::domain::di::service_ref::ServiceRef;
use crate::domain::model::downloaded_file::DownloadedFile;
use crate::domain::model::file::File;
use crate::domain::model::uploaded_file::UploadedFile;
use crate::domain::model::mime_type::MimeType;
use crate::domain::model::file_id::FileId;
use failure::Error;

#[derive(new)]
pub struct NamedFileFactory {
    filename_generator: ServiceRef<&'static dyn FileNameGeneratorService>,
}

impl NamedFileFactory {
    fn generate_filename(&self, mime_type: &MimeType,filename: Option<String>) -> Result<String, Error> {
        match filename {
            Some(filename) => {
                return Ok(filename);
            },
            None => {
                let filename = self.filename_generator.generate_filename(mime_type)?;
                return Ok(filename);
            },
        }
    }
}

impl NamedFileFactoryTrait for NamedFileFactory {
    fn from_downloaded_file(&self, downloaded_file: DownloadedFile) -> Result<File, Error> {
        let (path, mime_type, filename) = downloaded_file.into_components();
        let path = path.into_path();
        let filename = self.generate_filename(&mime_type, filename)?;
        let file = File::new(FileId::new(), path, filename);
        return Ok(file);
    }

    fn from_uploaded_file(&self, uploaded_file: UploadedFile) -> Result<File, Error> {
        let (path, mime_type, filename) = uploaded_file.into_components();
        let path = path.into_path();
        let filename = self.generate_filename(&mime_type, filename)?;
        let file = File::new(FileId::new(), path, filename);
        return Ok(file);
    }
}