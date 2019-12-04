use crate::domain::model::application_config::web_server::WebServer;
use crate::domain::model::application_config::image_converter::ImageConverter;
use std::path::PathBuf;

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationConfig {
    #[serde(default)]
    web_server: WebServer,
    storage_path: PathBuf,
    #[serde(default = "ApplicationConfig::default_user_agent")]
    user_agent: String,
    #[serde(default = "ApplicationConfig::default_image_converter")]
    image_converter: ImageConverter,
}

impl ApplicationConfig {

    pub fn get_web_server(&self) -> &WebServer {
        return &self.web_server;
    }
    pub fn get_storage_path(&self) -> &PathBuf {
        return &self.storage_path;
    }

    pub fn get_user_agent(&self) -> &String {
        return &self.user_agent;
    }

    pub fn get_image_converter(&self) -> &ImageConverter {
        return &self.image_converter;
    }

    fn default_user_agent() -> String {
        format!("Robot-Test-Task-TrLogic/{} {}", env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_REPOSITORY"))
    }

    fn default_image_converter() -> ImageConverter {
        return ImageConverter::default();
    }
}