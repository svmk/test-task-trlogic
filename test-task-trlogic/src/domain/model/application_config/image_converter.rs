use std::default::Default;
use crate::domain::model::image_convert_config::ImageConvertConfig;

#[derive(Deserialize, Debug, Clone)]
pub struct ImageConverter {
    #[serde(default = "ImageConverter::default_preview")]
    preview: ImageConvertConfig,
}

impl ImageConverter {
    pub fn get_preview(&self) -> &ImageConvertConfig {
        return &self.preview;
    }

    fn default_preview() -> ImageConvertConfig {
        return ImageConvertConfig::new_resize(100, 100);
    }
}

impl Default for ImageConverter {
    fn default() -> Self {
        return ImageConverter {
            preview: ImageConverter::default_preview(),
        }
    }
}