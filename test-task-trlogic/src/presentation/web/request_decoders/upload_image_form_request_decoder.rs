use warp::filters::multipart::{FormData};
use crate::domain::model::url::Url;
use crate::domain::model::mime_type::MimeType;
use crate::domain::model::uploaded_file::UploadedFile;

use crate::presentation::web::request::upload_image_request::UploadImageRequest;
use crate::presentation::web::utils::form_parts::FormPart;
use std::str::FromStr;
use failure::{Error, err_msg};
use futures_old::{Future, Stream};
use futures_old::future::result as future_result;
use warp::{Filter, Rejection};
use warp::reject::custom;

#[derive(new, Clone)]
pub struct UploadImageFormRequestDecoder {

}

impl UploadImageFormRequestDecoder {
    pub fn create_decoder(&self) -> impl Filter<Extract = (UploadImageRequest,),Error=Rejection> + Clone {
        let this = self.clone();
        return warp::filters::multipart::form()
            .and_then(move |form| {
                return this
                    .decode_body(form)
                    .map_err(custom);
            });
    }

    fn decode_body(&self, form: FormData) -> impl Future<Item = UploadImageRequest, Error=Error> {
        let this = self.clone();
        let form_data = form
            .map_err(err_msg)
            .and_then(FormPart::from_warp_form_part)
            .collect()
            .and_then(move |parts| {
                return future_result(this.decode_form(parts));
            });
        return form_data;
    }

    fn decode_form(&self, mut parts: Vec<FormPart>) -> Result<UploadImageRequest, Error> {
        let mut result = UploadImageRequest::new();
        for part in parts.drain(..) {
            match part.get_name().as_str() {
                UploadImageRequest::FILES_FIELD => {
                    let content_type = part
                        .get_content_type()
                        .map(|content_type| {content_type.clone()})
                        .unwrap_or_else(|| { return MimeType::new_application_octet_stream(); });
                    let file = part.into_file_content()?;
                    let filename = file.get_filename().to_owned();
                    let file = UploadedFile::new(
                        file.into_temp_file_path(),
                        content_type
                    ).with_filename(filename);
                    result.append_file(file);
                },
                UploadImageRequest::FILE_EXTERNAL_URLS_FIELD => {
                    let url = part.into_text_content()?;
                    let url = Url::from_str(url.get_content())?;
                    result.append_url(url);
                },
                _ => {},
            }
        }
        return Ok(result);
    }
}

