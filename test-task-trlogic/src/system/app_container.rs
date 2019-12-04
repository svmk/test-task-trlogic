use crate::domain::model::application_config::ApplicationConfig;
use crate::domain::di::service_builder::ServiceBuilder;
use crate::infrastructure::repository::file_repository::FileRepository;
use crate::infrastructure::service::file_downloading::FileDownloading;
use crate::infrastructure::service::filename_generator::FileNameGenerator;
use crate::infrastructure::service::image_converter::ImageConverter;
use crate::infrastructure::service::mime_type_detector::MimeTypeDetector;
use crate::application::factory::named_file_factory::NamedFileFactory;
use crate::domain::service::filename_generator::FileNameGeneratorService;
use crate::application::service::url_generator::UrlGenerator;
use failure::Error;

pub struct AppContainer {
    pub config: ApplicationConfig,
    pub file_repository: ServiceBuilder<Self, FileRepository>,
    pub file_downloading: ServiceBuilder<Self, FileDownloading>,
    pub filename_generator: ServiceBuilder<Self, FileNameGenerator>,
    pub named_file_factory: ServiceBuilder<Self, NamedFileFactory>,
    pub url_generator: ServiceBuilder<Self, UrlGenerator>,
    pub mime_type_detector: ServiceBuilder<Self, MimeTypeDetector>,
    pub image_converter: ServiceBuilder<Self, ImageConverter>,
}

impl AppContainer {
    pub fn new(config: ApplicationConfig) -> Result<AppContainer, Error> {
        return Ok(AppContainer {
            config,
            file_repository: ServiceBuilder::new(Self::init_file_repository),
            file_downloading: ServiceBuilder::new(Self::init_file_downloading),
            filename_generator: ServiceBuilder::new(Self::init_filename_generator),
            named_file_factory: ServiceBuilder::new(Self::init_named_file_factory),
            url_generator: ServiceBuilder::new(Self::init_url_generator),
            mime_type_detector: ServiceBuilder::new(Self::init_mime_type_detector),
            image_converter: ServiceBuilder::new(Self::init_image_converter),
        })
    }

    pub fn init_file_repository(&self) -> Result<FileRepository, Error> {
        let repository = FileRepository::new(self.config.get_storage_path().clone())?;
        return Ok(repository);
    }

    pub fn init_file_downloading(&self) -> Result<FileDownloading, Error> {
        let service = FileDownloading::new(self.config.get_user_agent())?;
        return Ok(service);
    }

    pub fn init_filename_generator(&self) -> Result<FileNameGenerator, Error> {
        let service = FileNameGenerator::new();
        return Ok(service);
    }

    pub fn init_named_file_factory(&self) -> Result<NamedFileFactory, Error> {
        let service = NamedFileFactory::new(
            crate::service_ref!(self.filename_generator.get(self)? => &dyn FileNameGeneratorService),
        );
        return Ok(service);
    }

    pub fn init_url_generator(&self) -> Result<UrlGenerator, Error> {
        let base_url = self.config.get_web_server().get_base_url();
        let service = UrlGenerator::new(base_url.clone());
        return Ok(service);
    }

    pub fn init_mime_type_detector(&self) -> Result<MimeTypeDetector, Error> {
        let service = MimeTypeDetector::new();
        return Ok(service);
    }

    pub fn init_image_converter(&self) -> Result<ImageConverter, Error> {
        let service = ImageConverter::new();
        return Ok(service);
    }
}