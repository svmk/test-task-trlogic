use warp::{Rejection, Filter, reject::not_found};

pub fn header_match(name: &'static str, value: &'static str) -> impl Filter<Extract=((),), Error=Rejection> + Copy {
    let filter = warp::filters::header::optional::<String>(name);
    return filter.and_then(move |header_value| {
        if let Some(header_value) = header_value {
            if header_value == value {
                return Ok(());
            }
        }
        return Err(not_found());
    });
}