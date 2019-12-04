
use warp::reject::{Rejection, not_found};
use futures_old::Future;

pub fn maybe_found_response_encode<T>(response: Option<T>) -> impl Future<Item=T, Error=Rejection> {
    if let Some(response) = response {
        return futures_old::future::ok(response);
    } else {
        return futures_old::future::err(not_found());
    }
}