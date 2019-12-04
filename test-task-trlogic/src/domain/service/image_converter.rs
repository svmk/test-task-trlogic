use crate::domain::model::file::File;
use crate::domain::model::temp_file_path::TempFilePath;
use crate::domain::model::image_convert_config::ImageConvertConfig;
use failure::Error;

pub trait ImageConverterService {
    fn convert_image(&self, source: &File, config: &ImageConvertConfig) -> Result<TempFilePath, Error>;
}