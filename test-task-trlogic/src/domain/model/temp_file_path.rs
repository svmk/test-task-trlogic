use crate::domain::model::path_resolve::PathResolve;
use std::path::{PathBuf, Path};
use std::ops::Drop;
use std::fs::remove_file;

#[derive(Debug)]
pub struct TempFilePath {
    path: Option<PathBuf>,
}

impl TempFilePath {
    pub fn from_path(path: PathBuf) -> TempFilePath {
        return TempFilePath {
            path: Some(path),
        }
    }

    pub fn get_path(&self) -> &PathBuf {
        return self.path.as_ref().unwrap();
    }

    pub fn into_path(mut self) -> PathBuf {
        return self.path.take().unwrap();
    }
}

impl Drop for TempFilePath {
    fn drop(&mut self) {
        if let Some(ref path) = self.path {
            let _ = remove_file(path);
        }
    }
}

impl PathResolve for TempFilePath {
    fn get_path(&self) -> &Path {
        return self.path.as_ref().unwrap();
    }
}