use serde::Serialize;
use warp::Reply;
use warp::reply::json;


pub fn json_response_encode<T>(response: T) -> impl Reply where T: Serialize {
    json(&response)
}