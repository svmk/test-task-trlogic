use warp::{Rejection, Filter, reject::not_found, reject::custom};
use crate::domain::utils::get_content_type::GetContentType;
use http::header::HeaderMap;
pub fn content_type_match(content_type: &'static str) -> impl Filter<Extract=((),), Error=Rejection> + Copy {
    let filter = warp::filters::header::headers_cloned();
    return filter.and_then(move |headers: HeaderMap| {
        let request_content_type = headers.get_content_type().map_err(custom)?;
        if let Some(request_content_type) = request_content_type {
            if request_content_type.as_str() == content_type {
                return Ok(());
            }
        }
        return Err(not_found());
    });
}