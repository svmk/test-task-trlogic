use warp::reject::{Rejection, custom};
use crate::presentation::web::error::WebError;
use futures_old::Future;
use futures_old::future::result as future_result;

pub fn result_response_encode<T>(result: Result<T, WebError>) -> impl Future<Item=T, Error=Rejection> {
    return future_result(result.map_err(custom));
}