use crate::domain::model::mime_type::MimeType;
use crate::domain::model::temp_file_path::TempFilePath;

#[derive(Debug)]
pub struct UploadedFile {
    path: TempFilePath,
    mime_type: MimeType,
    name: Option<String>,
}

impl UploadedFile {
    pub fn new(path: TempFilePath, mime_type: MimeType) -> UploadedFile {
        return UploadedFile {
            path,
            mime_type,
            name: None,
        }
    }

    pub fn with_filename(mut self, name: String) -> Self {
        self.name = Some(name);
        return self;
    }

    pub fn into_components(self) -> (TempFilePath, MimeType, Option<String>) {
        return (self.path, self.mime_type, self.name);
    }

    pub fn get_mime_type(&self) -> &MimeType {
        return &self.mime_type;
    }
}
