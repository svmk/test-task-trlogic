use http::header::HeaderMap;
use http::header::CONTENT_TYPE;
use failure::Error;
use crate::domain::utils::get_content_type::GetContentType;

impl GetContentType for HeaderMap {
    fn get_content_type_text(&self) -> Result<Option<String>, Error> {
        if let Some(content_type_header) = self.get(CONTENT_TYPE) {
            let content_type_header = content_type_header.to_str()?;
            let content_type_header = content_type_header.to_string();
            return Ok(Some(content_type_header));
        }
        return Ok(None);
    }
}