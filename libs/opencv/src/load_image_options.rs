use std::default::Default;
use libc::c_int;

pub struct LoadImageOptions(c_int);

impl LoadImageOptions {
    //    const CV_LOAD_IMAGE_UNCHANGED: c_int = -1;
//    const CV_LOAD_IMAGE_GRAYSCALE: c_int = 0;
    const CV_LOAD_IMAGE_COLOR: c_int = 1;
//    const CV_LOAD_IMAGE_ANYDEPTH: c_int = 2;
//    const CV_LOAD_IMAGE_ANYCOLOR: c_int = 4;
//    const CV_LOAD_IMAGE_IGNORE_ORIENTATION: c_int = 128;

    pub (in crate) fn get_value(&self) -> c_int {
        return self.0;
    }
}

impl Default for LoadImageOptions {
    fn default() -> Self {
        return LoadImageOptions(Self::CV_LOAD_IMAGE_COLOR);
    }
}