use std::path::Path;
use crate::domain::model::path_resolve::PathResolve;

pub struct FileResponse {
    path: Box<dyn PathResolve + Sync + Send>,
    response_type: FileResponseType,
}

pub enum FileResponseType {
    Attachment(String),
    Inline,
}

impl FileResponse {
    pub fn attachment(path: impl PathResolve  + Sync + Send + 'static, filename: String) -> FileResponse {
        let response_type = FileResponseType::Attachment(filename);
        return FileResponse {
            path: Box::new(path),
            response_type,
        };
    }

    pub fn inline(path: impl PathResolve  + Sync + Send + 'static) -> FileResponse {
        let response_type = FileResponseType::Inline;
        return FileResponse {
            path: Box::new(path),
            response_type,
        };
    }

    pub fn get_path(&self) -> &Path {
        return self.path.get_path();
    }

    pub fn get_response_type(&self) -> &FileResponseType {
        return &self.response_type;
    }
}