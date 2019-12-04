use crate::{Error, ImageSize, PixelDepth, ImageChannels, LoadImageOptions, Interpolation};
use crate::sys;
use std::path::Path;
use std::ffi::CString;
use std::fmt;


pub struct Image(*const sys::IplImage);

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let this = self.get_this();
        write!(f, "Image {} *{:?} {}", "{", this, "}")
    }
}

//unsafe impl Send for Image {}

impl Image {
    fn new(size: ImageSize, depth: PixelDepth, channels: ImageChannels) -> Image {
        let image = unsafe {
            sys::cvCreateImage(size, depth.as_c_int(), channels.as_c_int())
        };
        return Image(image);
    }

    fn get_this(&self) -> &sys::IplImage {
        unsafe {
            return self.0.as_ref().unwrap();
        }
    }

    pub fn open(path: &Path, options: LoadImageOptions) -> Result<Image, Error> {
        let path = path.to_str().ok_or_else(|| {
            return Error::PathConvertToUnicode;
        })?;
        let path = CString::new(path.to_owned().into_bytes())
            .map_err(|error| {
                return Error::Convert(error.into());
            })?;
        unsafe {
            if sys::cvHaveImageReader(path.as_ptr()) == 0 {
                return Err(Error::UnsupportedImageReader);
            }
        }
        let image = unsafe {
            sys::cvLoadImage(path.as_ptr(), options.get_value())
        };
        if image.is_null() {
            return Err(Error::UnableOpenFile);
        }
        return Ok(Image(image));
    }

    pub fn resize(&self, new_size: ImageSize, interpolation: Interpolation) -> Image {
        let this = self.get_this();
        let new_image = Image::new(new_size, PixelDepth::from_c_int(this.depth), ImageChannels::from_c_int(this.n_channels));
        unsafe {
            sys::cvResize(self.0 as sys::CvArr, new_image.0 as sys::CvArr, interpolation.as_c_int());
        }
        return new_image
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let path = path.to_str().ok_or_else(|| {
            return Error::PathConvertToUnicode;
        })?;
        let path = CString::new(path.to_owned().into_bytes())
            .map_err(|error| {
                return Error::Convert(error.into());
            })?;
        unsafe {
            if sys::cvHaveImageWriter(path.as_ptr()) == 0 {
                return Err(Error::UnsupportedImageWriter);
            }
        }
        let result = unsafe {
            sys::cvSaveImage(path.as_ptr(), self.0 as sys::CvArr, std::ptr::null())
        };
        if result != 1 {
            return Err(Error::UnableSaveFile);
        }
        return Ok(());
    }
}