#[derive(Deserialize, Debug, Clone)]
pub struct ImageConvertConfig {
    width: u16,
    height: u16,
}

impl ImageConvertConfig {
    pub fn new_resize(width: u16, height: u16) -> ImageConvertConfig {
        return ImageConvertConfig {
            width,
            height,
        }
    }
    pub fn get_width(&self) -> u16 {
        return self.width;
    }

    pub fn get_height(&self) -> u16 {
        return self.height;
    }
}