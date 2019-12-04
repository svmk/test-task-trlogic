use crate::domain::model::file::File;
use crate::domain::model::file_id::FileId;
use crate::domain::repository::file_repository::FileRepository as FileRepositoryTrait;
use failure::{Error, err_msg};
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileRepository {
    base_path: PathBuf,
}

impl FileRepository {
    const SYMLINK_FILENAME: &'static str = "symlink";

    pub fn new(base_path: PathBuf) -> Result<FileRepository, Error> {
        std::fs::create_dir_all(&base_path)?;
        let base_path = base_path.canonicalize()?;
        return Ok(FileRepository {
            base_path,
        })
    }
    fn get_directory_path(&self, file_id: &FileId) -> PathBuf {
        let mut path = self.base_path.clone();
        let file_id = file_id.to_string();
        path.push(&file_id[0..2]);
        path.push(&file_id[2..4]);
        path.push(&file_id);
        return path;
    }
}

impl FileRepositoryTrait for FileRepository {
    fn get(&self, file_id: &FileId) -> Result<Option<File>, Error> {
        let symlink_path = self.get_directory_path(file_id).join(Self::SYMLINK_FILENAME);
        if !symlink_path.is_file() {
            return Ok(None);
        }
        let real_path = std::fs::canonicalize(symlink_path)?;
        if !real_path.is_file() {
            return Ok(None);
        }
        let name = real_path.file_name().ok_or_else(|| {
            return err_msg("Unable to get filename");
        })?;
        let name = name.to_os_string().into_string().map_err(|_| {
            return err_msg("Unable to convert os string into String");
        })?;
        let file = File::new(file_id.clone(), real_path, name);
        return Ok(Some(file));
    }

    fn add(&self, file: File) -> Result<File, Error> {
        let directory_path = self.get_directory_path(file.get_id());
        if directory_path.exists() {
            return Err(err_msg(format!("File with id `{}` already exists", file.get_id())));
        }
        std::fs::create_dir_all(&directory_path)?;
        let destination_path = directory_path.join(file.get_filename());
        let symlink_path = directory_path.join(Self::SYMLINK_FILENAME);
        std::fs::copy(file.get_path(), &destination_path)?;
        std::fs::remove_file(file.get_path())?;
        std::os::unix::fs::symlink(&destination_path, symlink_path)?;
        let file = file.with_path(destination_path);
        return Ok(file);
    }
}