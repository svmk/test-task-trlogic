use reqwest::r#async::Response;
use reqwest::header::CONTENT_TYPE;
use crate::domain::utils::get_content_type::GetContentType;

use failure::{Error, err_msg};

impl GetContentType for Response {
    fn get_content_type_text(&self) -> Result<Option<String>, Error> {
        if let Some(content_type) = self.headers().get(CONTENT_TYPE) {
            let content_type = content_type.to_str().map_err(err_msg)?;
            return Ok(Some(content_type.to_string()));
        }
        return Ok(None);
    }
}