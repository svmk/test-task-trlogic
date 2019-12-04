use failure::Error;
use crate::domain::model::mime_type::MimeType;
use std::str::FromStr;

pub trait GetContentType {
    fn get_content_type_text(&self) -> Result<Option<String>, Error>;
    fn get_content_type(&self) -> Result<Option<MimeType>, Error> {
        if let Some(content_type) = self.get_content_type_text()? {
            let mime_type = content_type.split(';').nth(0).unwrap();
            let mime_type = mime_type.trim();
            let mime_type = MimeType::from_str(mime_type)?;
            return Ok(Some(mime_type));
        }
        return Ok(None);
    }
}