use crate::domain::model::file_id::FileId;
use std::path::{PathBuf, Path};

#[derive(Debug)]
pub struct File {
    file_id: FileId,
    path: PathBuf,
    filename: String,
}

impl File {
    pub fn new(file_id: FileId, path: PathBuf, filename: String) -> File {
        return File {
            file_id,
            path,
            filename,
        }
    }

    pub fn with_path(mut self, path: PathBuf) -> File {
        self.path = path;
        return self;
    }

    pub fn get_id(&self) -> &FileId {
        return &self.file_id;
    }

    pub fn get_path(&self) -> &PathBuf {
        return &self.path;
    }

    pub fn get_filename(&self) -> &String {
        return &self.filename;
    }

    pub fn get_filename_ext(&self) -> Option<&str> {
        let filename = Path::new(&self.filename);
        return filename
            .extension()
            .and_then(|ext| {
                return ext.to_str();
            });
    }
}