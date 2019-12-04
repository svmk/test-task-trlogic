use crate::domain::model::mime_type::MimeType;
use crate::domain::model::url::Url;
use crate::domain::model::temp_file_path::TempFilePath;

#[derive(Debug)]
pub struct DownloadedFile {
    fetched_url: Url,
    file: TempFilePath,
    mime_type: MimeType,
    name: Option<String>,
}

impl DownloadedFile {
    pub fn new(fetched_url: Url, file: TempFilePath, mime_type: MimeType) -> DownloadedFile {
        return DownloadedFile {
            fetched_url,
            file,
            mime_type,
            name: None,
        }
    }

    pub fn with_filename(mut self, name: String) -> Self {
        self.name = Some(name);
        return self;
    }

    pub fn into_components(self) -> (TempFilePath, MimeType, Option<String>) {
        return (self.file, self.mime_type, self.name);
    }

    pub fn get_mime_type(&self) -> &MimeType {
        return &self.mime_type;
    }

    pub fn get_fetched_url(&self) -> &Url {
        return &self.fetched_url;
    }
}