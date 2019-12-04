use crate::domain::service::file_downloading::FileDownloadingService;
use crate::domain::model::url::Url;
use crate::domain::model::temp_file_path::TempFilePath;
use crate::domain::model::downloaded_file::DownloadedFile;
use crate::domain::model::mime_type::MimeType;
use crate::domain::utils::get_content_type::GetContentType;
use std::fs::File;
use std::str::FromStr;
use reqwest::r#async::{Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_DISPOSITION};
use failure::{Error, err_msg};
use std::iter::Iterator;
use std::io::Write;

pub struct FileDownloading {
    http_client: Client,
}

impl FileDownloading {
    pub fn new(user_agent: &str) -> Result<FileDownloading, Error> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent).map_err(err_msg)?);
        let http_client = Client::builder()
            .default_headers(headers)
            .gzip(true)
            .build()
            .map_err(err_msg)?;
        return Ok(FileDownloading {
            http_client,
        });
    }

    async fn copy_response_body_to_file(response: &mut Response, file: &mut File) -> Result<(), Error> {
        use futures::compat::Stream01CompatExt;
        use futures::stream::StreamExt;
        let mut body = response.body_mut().compat();
        while let Some(chunk) = body.next().await {
            let chunk = chunk?;
            file.write(chunk.as_ref()).map_err(err_msg)?;
        }
        return Ok(());
    }
}

#[async_trait]
impl FileDownloadingService for FileDownloading {
    async fn download_file(&self, url: &Url) -> Result<DownloadedFile, Error> {
        use futures::compat::Future01CompatExt;
        let fetched_url = url.clone();
        let url = url1::Url::from_str(url.as_str())?;
        let request = self
            .http_client
            .get(url)
            .build()?;
        let mut response = self
            .http_client
            .execute(request)
            .compat()
            .await?;
        let filename = get_filename(&response)?;
        let temp_path = TempFilePath::new()?;
        let mut file = File::create(temp_path.get_path())?;
        let mime_type = response.get_content_type()?.unwrap_or(MimeType::new_application_octet_stream());
        Self::copy_response_body_to_file(&mut response, &mut file).await?;
        let mut downloaded_file = DownloadedFile::new(fetched_url, temp_path, mime_type);
        if let Some(filename) = filename {
            downloaded_file = downloaded_file.with_filename(filename);
        }
        return Ok(downloaded_file);
    }
}

fn content_disposition_kv(line: &str) -> Result<(String, String), Error> {
    let parts: Vec<_> = line.split('=').collect();
    if parts.len() != 2 {
        return Err(err_msg("Unable to parse content-disposition header: invalid number of kv"));
    }
    let key: String = parts[0].trim().into();
    let value: String = parts[1].trim().trim_matches('"').into();
    return Ok((key, value));
}

fn get_filename(response: &Response) -> Result<Option<String>, Error> {
    if let Some(content_disposition) = response.headers().get(CONTENT_DISPOSITION) {
        let content_disposition = content_disposition.to_str().map_err(err_msg)?;
        let mut tokens: Vec<_> = content_disposition.split(';').collect();
        if tokens.len() >= 1 {
            // may be inline, attachment etc
            let _ = tokens.remove(0);
        }
        for token in tokens.drain(..) {
            let (key, value) = content_disposition_kv(&token)?;
            if key == "filename" {
                return Ok(Some(value));
            }
        }
    }
    let path = std::path::Path::new(response.url().path());
    if let Some(path) = path.file_name() {
        return Ok(path.to_str().map(String::from));
    }
    return Ok(None);
}