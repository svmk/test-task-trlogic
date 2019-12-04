use crate::presentation::web::error::WebError;
use crate::domain::model::file_id::FileId;
use crate::domain::repository::file_repository::FileRepository;
use crate::domain::di::service_ref::ServiceRef;
use crate::presentation::web::response::file_response::FileResponse;

#[derive(new)]
pub struct FetchFileController {
    file_repository: ServiceRef<&'static dyn FileRepository>,
}

impl FetchFileController {
    pub fn fetch_file(&self, file_id: FileId) -> Result<Option<FileResponse>, WebError> {
        let file = self.file_repository.get(&file_id)?;
        if let Some(file) = file {
            let file_response = FileResponse::attachment(
                file.get_path().clone(),
                file.get_filename().clone(),
            );
            return Ok(Some(file_response));
        }
        return Ok(None);
    }
}