use mime::{APPLICATION_OCTET_STREAM, MULTIPART_FORM_DATA, APPLICATION_WWW_FORM_URLENCODED, APPLICATION_JSON};

use std::fmt;
use std::str::FromStr;
use std::convert::From;
use failure::{Error, err_msg};

#[derive(Debug, Clone)]
pub struct MimeType(mime::Mime);

impl MimeType {
    pub fn new_application_octet_stream() -> MimeType {
        return MimeType(APPLICATION_OCTET_STREAM);
    }

    pub fn is_form(&self) -> bool {
        return self.0 == APPLICATION_WWW_FORM_URLENCODED || self.0 == MULTIPART_FORM_DATA;
    }

    pub fn is_json(&self) -> bool {
        return self.0 == APPLICATION_JSON;
    }

    pub fn is_image(&self) -> bool {
        return self.0.type_() == mime::IMAGE;
    }

    pub fn as_str(&self) -> &str {
        return self.0.as_ref();
    }
}

impl <'a>Into<&'a mime::Mime> for &'a MimeType {
    fn into(self) -> &'a mime::Mime {
        return &self.0;
    }
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

impl FromStr for MimeType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mime_type = mime::Mime::from_str(s).map_err(err_msg)?;
        return Ok(MimeType(mime_type));
    }
}

impl From<mime::Mime> for MimeType {
    fn from(value: mime::Mime) -> Self {
        return MimeType(value);
    }
}