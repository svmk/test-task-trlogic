use crate::domain::service::image_converter::ImageConverterService;
use crate::domain::model::file::File;
use crate::domain::model::temp_file_path::TempFilePath;
use crate::domain::model::image_convert_config::ImageConvertConfig;
use failure::{Error, err_msg};

#[derive(new)]
pub struct ImageConverter {

}

impl ImageConverterService for ImageConverter {
    fn convert_image(&self, source: &File, config: &ImageConvertConfig) -> Result<TempFilePath, Error> {
        let image = opencv::Image::open(source.get_path(), Default::default())
            .map_err(err_msg)?;
        let image_size = opencv::ImageSize::new(config.get_height(), config.get_width());
        let image = image.resize(image_size, opencv::Interpolation::linear());
        let filename = source.get_path().file_name().ok_or_else(|| {
            err_msg("Unable to get filename for image converting")
        })?;
        let filename = filename.to_str().ok_or_else(|| {
            err_msg("Unable to get filename for image converting")
        })?;
        let temp_file = TempFilePath::new_with_filename(filename)?;
        image.save(temp_file.get_path()).map_err(err_msg)?;
        return Ok(temp_file);
    }
}