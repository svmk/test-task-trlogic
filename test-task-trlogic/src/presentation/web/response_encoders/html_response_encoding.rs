use warp::reply::Reply;
use crate::presentation::web::response::html_response::HtmlResponse;
use warp::reply::html as html_reply;

pub fn html_response_encode(response: HtmlResponse) -> impl Reply {
    let mut result = response.get_content().to_owned();
    for (key, value) in response.get_params().iter() {
        let key = format!("{}{}{}", "{{", key, "}}");
        result = result.replace(&key, value);
    }
    return html_reply(result)
}