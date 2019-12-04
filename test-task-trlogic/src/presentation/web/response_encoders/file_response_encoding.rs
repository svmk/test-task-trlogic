use crate::presentation::web::response::file_response::{FileResponse, FileResponseType};
use warp::{Reply, Rejection};
use warp::reject::custom as custom_error;
use futures_old::{Future, Stream};
use futures_old::future::ok as future_ok;
use tokio_fs::File as AsyncFile;
use tokio::codec::{BytesCodec, Decoder};
use hyper::Body;
use warp::http::Response;

pub fn file_response_encode(response: FileResponse) -> impl Future<Item=impl Reply, Error=Rejection> {

    let file_reply = AsyncFile::open(response.get_path().to_owned())
        .and_then(|file| {
            let bytes_codec = BytesCodec::new();
            let file_stream = bytes_codec.framed(file);
            let file_stream = file_stream.map(|chunk| {
                return chunk.freeze();
            });
            let hyper_body = Body::wrap_stream(file_stream);
            let file_reply = FileReply(hyper_body);
            return future_ok(file_reply);
        });
    let filter = file_reply
        .map_err(custom_error)
        .map(move |file_reply| {
            let content_disposition_header = match response.get_response_type() {
                &FileResponseType::Attachment(ref filename) => {
                    format!("attachment; filename=\"{}\"", filename)
                },
                &FileResponseType::Inline => {
                    "inline".into()
                },
            };
            return warp::reply::with_header(file_reply, "content-disposition", &content_disposition_header);
        });
    return filter;
}

struct FileReply(Body);

impl Reply for FileReply {
    fn into_response(self) -> Response<Body> {
        return Response::new(self.0);
    }
}