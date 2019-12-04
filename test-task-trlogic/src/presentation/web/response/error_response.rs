#[derive(Serialize)]
enum ErrorResponseType {
    #[serde(rename = "not_found")]
    NotFound,
    #[serde(rename = "internal_error")]
    InternalError,
    #[serde(rename = "unknown_error")]
    UnknownError,
    #[serde(rename = "validation_error")]
    ValidationError,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    error_type: ErrorResponseType,
    error: String,
}

impl ErrorResponse {
    pub fn not_found(error: String) -> ErrorResponse {
        return ErrorResponse {
            error,
            error_type: ErrorResponseType::NotFound,
        }
    }

    pub fn internal_error(error: String) -> ErrorResponse {
        return ErrorResponse {
            error,
            error_type: ErrorResponseType::InternalError,
        }
    }

    pub fn unknown_error(error: String) -> ErrorResponse {
        return ErrorResponse {
            error,
            error_type: ErrorResponseType::UnknownError,
        }
    }

    pub fn validation_error(error: String) -> ErrorResponse {
        return ErrorResponse {
            error,
            error_type: ErrorResponseType::ValidationError,
        }
    }
}