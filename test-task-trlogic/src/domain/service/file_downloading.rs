use crate::domain::model::url::Url;
use crate::domain::model::downloaded_file::DownloadedFile;
use async_trait::async_trait;
use failure::Error;

#[async_trait]
pub trait FileDownloadingService: Sync {
    async fn download_file(&self, url: &Url) -> Result<DownloadedFile, Error>;
}