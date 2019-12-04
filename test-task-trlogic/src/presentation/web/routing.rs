use crate::domain::model::application_config::WebServer;
use crate::domain::model::file_id::FileId;
use crate::domain::di::service::Service;
use crate::presentation::web::controller::main_page_controller::MainPageController;
use crate::presentation::web::controller::error_controller::ErrorController;
use crate::presentation::web::controller::upload_image_controller::UploadImageController;
use crate::presentation::web::controller::fetch_file_controller::FetchFileController;
use crate::presentation::web::controller::preview_image_controller::PreviewImageController;
use crate::presentation::web::request_decoders::upload_image_form_request_decoder::UploadImageFormRequestDecoder;
use crate::presentation::web::response_encoders::{html_response_encode, result_response_encode, json_response_encode, file_response_encode, maybe_found_response_encode};
use crate::presentation::web::filters::{header_match, content_type_match};
use futures::compat::Compat;
use futures_old::future::Future;
use warp::filters::BoxedFilter;
use warp::{Filter};
use warp::Reply;
use warp::filters;
use warp::reject::Rejection;


#[derive(new, Clone)]
pub struct Routing {
    web_config: WebServer,
    upload_image_request_decoder: Service<UploadImageFormRequestDecoder>,
    main_page_controller: Service<MainPageController>,
    error_page_controller: Service<ErrorController>,
    upload_image_controller: Service<UploadImageController>,
    fetch_file_controller: Service<FetchFileController>,
    preview_image_controller: Service<PreviewImageController>,
}

impl Routing {
    fn create_controller(&'static self) -> BoxedFilter<()> {
        let mut filter = filters::any::any().boxed();
        if let Some(domain) = self.web_config.get_base_url().get_domain() {
            filter = filter.and(header_match("HostName", domain)).untuple_one().boxed();
        }
        let path_parts = self.web_config.get_base_url().get_path().split('/');
        for path_part in path_parts {
            if !path_part.is_empty() {
                filter = filter.and(filters::path::path(path_part)).boxed();
            }
        }

//        let filter = filter.and(filters::header::headers_cloned())
//            .and_then(move |headers| {
//                println!("headers = {:?}", headers);
//                return futures_old::future::ok::<(), warp::reject::Rejection>(());
//            }).untuple_one().boxed();
        return filter;
    }

    fn main_page_route(&'static self) -> BoxedFilter<(impl Reply,)> {
        self.create_controller()
            .and(filters::path::end())
            .and(filters::method::get2())
            .map(move || {
                self.main_page_controller.index()
            })
            .and_then(result_response_encode)
            .map(html_response_encode)
            .boxed()
    }

    fn upload_image_route_json(&'static self) -> BoxedFilter<(impl Reply,)> {
        self.create_controller()
            .and(warp::path!("api"/"v1"/"image"/"upload"))
            .and(filters::path::end())
            .and(filters::method::post2())
            .and(content_type_match("application/json"))
            .untuple_one()
            .and(filters::body::json())
            .and_then(move |body| {
                let future = self.upload_image_controller.upload_image(body);
                let future = {
                    use futures::future::FutureExt;
                    future.boxed()
                };
                let future = Compat::new(future);
                let future = future.then(result_response_encode);
                return future;
            })
            .map(json_response_encode)
            .boxed()
    }

    fn upload_image_route_form(&'static self) -> BoxedFilter<(impl Reply,)> {
        self.create_controller()
            .and(warp::path!("api"/"v1"/"image"/"upload"))
            .and(filters::path::end())
            .and(filters::method::post2())
            .and(content_type_match("multipart/form-data"))
            .and(self.upload_image_request_decoder.create_decoder())
            .and_then(move |_, body| {
                let future = self.upload_image_controller.upload_image(body);
                let future = {
                    use futures::future::FutureExt;
                    future.boxed()
                };
                let future = Compat::new(future);
                let future = future.then(result_response_encode);
                return future;
            })
            .map(json_response_encode)
            .boxed()
    }

    fn fetch_file_route(&'static self) -> BoxedFilter<(impl Reply,)> {
        self.create_controller()
            .and(warp::path!("api"/"v1"/"files"/FileId))
            .and(filters::path::end())
            .and(filters::method::get2())
            .map(move |file_id| {
                self.fetch_file_controller.fetch_file(file_id)
            })
            .and_then(result_response_encode)
            .and_then(maybe_found_response_encode)
            .and_then(file_response_encode)
            .boxed()
    }

    fn preview_image_route(&'static self) -> BoxedFilter<(impl Reply,)> {
        self.create_controller()
            .and(warp::path!("api"/"v1"/"files"/FileId/"image-preview"))
            .and(filters::path::end())
            .and(filters::method::get2())
            .map(move |file_id| {
                self.preview_image_controller.preview_image(file_id)
            })
            .and_then(result_response_encode)
            .and_then(maybe_found_response_encode)
            .and_then(file_response_encode)
            .boxed()
    }

    pub fn create_routing(&'static self) -> impl Filter<Extract=impl Reply, Error=Rejection> {
        let application = self.main_page_route();
        let application = application.or(self.upload_image_route_form());
        let application = application.or(self.upload_image_route_json());
        let application = application.or(self.fetch_file_route());
        let application = application.or(self.preview_image_route());
        let application = application.recover(move |error| {
            let error = self.error_page_controller.handle_rejection(error);
            return Ok(error);
        });
        return application.boxed();
    }
}