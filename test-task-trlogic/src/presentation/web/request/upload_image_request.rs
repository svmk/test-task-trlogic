
use crate::domain::model::uploaded_file::UploadedFile;
use crate::domain::model::url::Url;

#[derive(Debug, Deserialize)]
pub struct UploadImageRequest {
    files: Vec<UploadedFile>,
    urls: Vec<Url>,
}

impl UploadImageRequest {
    pub const FILES_FIELD: &'static str = "files[]";
    pub const FILE_EXTERNAL_URLS_FIELD: &'static str = "urls[]";
    pub fn new() -> UploadImageRequest {
        UploadImageRequest {
            files: Vec::new(),
            urls: Vec::new(),
        }
    }

    pub fn append_file(&mut self, file: UploadedFile) {
        self.files.push(file);
    }

    pub fn append_url(&mut self, url: Url) {
        self.urls.push(url);
    }

    pub fn iter_urls(&self) -> impl Iterator<Item=&Url> {
        return self.urls.iter();
    }

    pub fn iter_uploaded_files(&self) -> impl Iterator<Item=&UploadedFile> {
        return self.files.iter();
    }

    pub fn into_uploaded_files(self) -> Vec<UploadedFile> {
        return self.files;
    }

    pub fn has_uploaded_files(&self) -> bool {
        return !self.files.is_empty();
    }

    pub fn has_urls(&self) -> bool {
        return !self.urls.is_empty();
    }
}