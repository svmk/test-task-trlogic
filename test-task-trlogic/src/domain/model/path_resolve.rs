use std::path::{Path, PathBuf};

pub trait PathResolve {
    fn get_path(&self) -> &Path;
}

impl PathResolve for Path {
    fn get_path(&self) -> &Path {
        return self;
    }
}

impl PathResolve for PathBuf {
    fn get_path(&self) -> &Path {
        return self;
    }
}