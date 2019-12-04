use crate::domain::model::file_id::FileId;
use crate::domain::model::url::Url;

#[derive(Serialize)]
pub struct UploadImageResponseItem {
    id: FileId,
    file_url: Url,
    preview_image_url: Url,
}

impl UploadImageResponseItem {
    pub fn new(id: FileId, file_url: Url, preview_image_url: Url) -> UploadImageResponseItem {
        return UploadImageResponseItem {
            id,
            file_url,
            preview_image_url,
        }
    }
}

#[derive(Serialize)]
pub struct UploadImageResponse {
    files: Vec<UploadImageResponseItem>,
}

impl UploadImageResponse {
    pub fn new() -> UploadImageResponse {
        return UploadImageResponse {
            files: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: UploadImageResponseItem) {
        self.files.push(item);
    }
}