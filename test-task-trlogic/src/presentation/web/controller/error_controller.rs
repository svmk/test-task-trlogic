use crate::presentation::web::response::error_response::ErrorResponse;
use crate::presentation::web::response_encoders::json_response_encode;
use crate::presentation::web::error::WebError;
use http::status::StatusCode;
use warp::{Rejection, Reply};

use warp::reply;


#[derive(new)]
pub struct ErrorController {

}

impl ErrorController {
    pub fn handle_rejection(&self, error: Rejection) -> impl Reply {
        if error.is_not_found() {
            let error = ErrorResponse::not_found("Resource not found".into());
            return reply::with_status(json_response_encode(error), StatusCode::NOT_FOUND);
        }
        if let Some(error) = error.find_cause::<WebError>() {
            match error {
                WebError::Failure(error) => {
                    let error = ErrorResponse::internal_error(format!("{}", error));
                    return reply::with_status(json_response_encode(error), StatusCode::INTERNAL_SERVER_ERROR);
                },
                WebError::Validation(error) => {
                    let error = ErrorResponse::validation_error(error.get_reason().into());
                    return reply::with_status(json_response_encode(error), StatusCode::BAD_REQUEST);
                },
                WebError::NotFound(error) => {
                    let error = ErrorResponse::not_found(error.get_reason().into());
                    return reply::with_status(json_response_encode(error), StatusCode::NOT_FOUND);
                },
            }
        }
        if let Some(error) = error.cause() {
            let error = ErrorResponse::internal_error(format!("{}", error));
            return reply::with_status(json_response_encode(error), StatusCode::INTERNAL_SERVER_ERROR);
        }
        let error = ErrorResponse::unknown_error("Unknown error".into());
        return reply::with_status(json_response_encode(error), StatusCode::INTERNAL_SERVER_ERROR);
    }
}