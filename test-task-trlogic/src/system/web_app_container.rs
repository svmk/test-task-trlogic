use crate::domain::model::application_config::ApplicationConfig;
use crate::system::app_container::AppContainer;
use crate::domain::di::service_builder::ServiceBuilder;
use crate::domain::di::service_ref::ServiceRef;
use crate::domain::factory::named_file_factory::NamedFileFactory;
use crate::domain::repository::file_repository::FileRepository;
use crate::domain::service::file_downloading::FileDownloadingService;
use crate::domain::service::url_generator::UrlGeneratorService;
use crate::domain::service::image_converter::ImageConverterService;
use crate::domain::service::mime_type_detector::MimeTypeDetectorService;
use crate::presentation::web::controller::upload_image_controller::UploadImageController;
use crate::presentation::web::controller::main_page_controller::MainPageController;
use crate::presentation::web::controller::error_controller::ErrorController;
use crate::presentation::web::controller::fetch_file_controller::FetchFileController;
use crate::presentation::web::controller::preview_image_controller::PreviewImageController;
use crate::presentation::web::request_decoders::upload_image_form_request_decoder::UploadImageFormRequestDecoder;
use crate::presentation::web::routing::Routing;
use failure::Error;

pub struct WebAppContainer {
    pub app_config: ApplicationConfig,
    pub file_downloading: ServiceRef<&'static dyn FileDownloadingService>,
    pub named_file_factory: ServiceRef<&'static dyn NamedFileFactory>,
    pub file_repository: ServiceRef<&'static dyn FileRepository>,
    pub url_generator: ServiceRef<&'static dyn UrlGeneratorService>,
    pub mime_type_detector: ServiceRef<&'static dyn MimeTypeDetectorService>,
    pub image_converter: ServiceRef<&'static dyn ImageConverterService>,
    pub upload_image_controller: ServiceBuilder<Self, UploadImageController>,
    pub main_page_controller: ServiceBuilder<Self, MainPageController>,
    pub error_page_controller: ServiceBuilder<Self, ErrorController>,
    pub upload_image_request_decoder: ServiceBuilder<Self, UploadImageFormRequestDecoder>,
    pub fetch_file_controller: ServiceBuilder<Self, FetchFileController>,
    pub preview_image_controller: ServiceBuilder<Self, PreviewImageController>,
    pub routing: ServiceBuilder<Self, Routing>,
}

impl WebAppContainer {
    pub fn from_application(application: &AppContainer) -> Result<WebAppContainer, Error> {
        return Ok(WebAppContainer {
            app_config: application.config.clone(),
            file_downloading: crate::service_ref!(application.file_downloading.get(application)? => &dyn FileDownloadingService),
            named_file_factory: crate::service_ref!(application.named_file_factory.get(application)? => &dyn NamedFileFactory),
            file_repository: crate::service_ref!(application.file_repository.get(application)? => &dyn FileRepository),
            url_generator: crate::service_ref!(application.url_generator.get(application)? => &dyn UrlGeneratorService),
            mime_type_detector: crate::service_ref!(application.mime_type_detector.get(application)? => &dyn MimeTypeDetectorService),
            image_converter: crate::service_ref!(application.image_converter.get(application)? => &dyn ImageConverterService),
            upload_image_controller: ServiceBuilder::new(Self::init_upload_image_controller),
            main_page_controller: ServiceBuilder::new(Self::init_main_page_controller),
            error_page_controller: ServiceBuilder::new(Self::init_error_page_controller),
            upload_image_request_decoder: ServiceBuilder::new(Self::init_upload_image_request_decoder),
            fetch_file_controller: ServiceBuilder::new(Self::init_fetch_file_controller),
            preview_image_controller: ServiceBuilder::new(Self::init_preview_image_controller),
            routing: ServiceBuilder::new(Self::init_routing),
        });
    }

    fn init_upload_image_controller(&self) -> Result<UploadImageController, Error> {
        let service = UploadImageController::new(
            self.file_downloading.clone(),
            self.named_file_factory.clone(),
            self.file_repository.clone(),
            self.url_generator.clone(),
        );
        return Ok(service);
    }

    fn init_main_page_controller(&self) -> Result<MainPageController, Error> {
        let service = MainPageController::new();
        return Ok(service);
    }

    fn init_error_page_controller(&self) -> Result<ErrorController, Error> {
        let service = ErrorController::new();
        return Ok(service);
    }

    fn init_upload_image_request_decoder(&self) -> Result<UploadImageFormRequestDecoder, Error> {
        let service = UploadImageFormRequestDecoder::new();
        return Ok(service);
    }

    fn init_fetch_file_controller(&self) -> Result<FetchFileController, Error> {
        let service = FetchFileController::new(self.file_repository.clone());
        return Ok(service);
    }

    fn init_preview_image_controller(&self) -> Result<PreviewImageController, Error> {
        let service = PreviewImageController::new(
            self.app_config.get_image_converter().clone(),
            self.file_repository.clone(),
            self.mime_type_detector.clone(),
            self.image_converter.clone(),
        );
        return Ok(service);
    }

    fn init_routing(&self) -> Result<Routing, Error> {
        let service = Routing::new(
            self.app_config.get_web_server().clone(),
            self.upload_image_request_decoder.get(&self)?,
            self.main_page_controller.get(&self)?,
            self.error_page_controller.get(&self)?,
            self.upload_image_controller.get(&self)?,
            self.fetch_file_controller.get(&self)?,
            self.preview_image_controller.get(&self)?,
        );
        return Ok(service);
    }
}