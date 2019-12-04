use libc::c_int;

#[repr(C)]
pub struct ImageSize {
    height: c_int,
    width: c_int,
}

impl ImageSize {
    pub fn new(height: u16, width: u16) -> ImageSize {
        return ImageSize {
            height: height as c_int,
            width: width as c_int,
        }
    }
}