use libc::c_int;

pub struct ImageChannels(c_int);

impl ImageChannels {
    pub (in crate) fn as_c_int(&self) -> c_int {
        return self.0;
    }

    pub (in crate) fn from_c_int(value: c_int) -> Self {
        return ImageChannels(value);
    }
}