use libc::c_int;

pub struct PixelDepth(c_int);

impl PixelDepth {
//    const IPL_DEPTH_SIGN: c_int = -0x80000000;
//    const IPL_DEPTH_1U : c_int =  1;
//    const IPL_DEPTH_8U : c_int =  8;
//    const IPL_DEPTH_16U: c_int = 16;
//    const IPL_DEPTH_32F: c_int = 32;
//    const IPL_DEPTH_8S : c_int = (Self::IPL_DEPTH_SIGN| 8);
//    const IPL_DEPTH_16S: c_int = (Self::IPL_DEPTH_SIGN|16);
//    const IPL_DEPTH_32S: c_int = (Self::IPL_DEPTH_SIGN|32);

    pub (in crate) fn as_c_int(&self) -> c_int {
        return self.0;
    }

    pub (in crate) fn from_c_int(value: c_int) -> Self {
        return PixelDepth(value);
    }
}
