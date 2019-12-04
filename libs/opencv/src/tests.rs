use crate::{Image, LoadImageOptions, ImageSize, Interpolation};
use crate::sys;
use std::default::Default;
use std::path::Path;
use std::time::SystemTime;

#[test]
fn test_size_of_equals() {
    use std::ffi::CString;
    let path: Vec<u8> = "data/img1.jpg".to_string().into_bytes();
    let path = CString::new(path).unwrap();
    let image = unsafe {
        sys::cvLoadImage(path.as_ptr(), crate::LoadImageOptions::default().get_value())
    };
    assert!(!image.is_null());
    let image = unsafe {
        image.as_ref().unwrap()
    };
    assert_eq!(std::mem::size_of::<sys::IplImage>() as i32, image.n_size);
}

#[test]
fn test_not_open() {
    let path = Path::new("data/not_exists.jpg");
    let image = Image::open(path, LoadImageOptions::default());
    assert!(image.is_err());
}

#[test]
fn test_convert() {
    let path = Path::new("data/img1.jpg");
    let image = Image::open(path, LoadImageOptions::default());
    assert!(image.is_ok());
    let image = image.unwrap();
    let image = image.resize(ImageSize::new(100, 100), Interpolation::linear());
    let target_directory = std::env::temp_dir().join("test-images");
    std::fs::create_dir_all(&target_directory).unwrap();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let filename = target_directory.join(format!("{}_100x100.png", now.as_secs()));
    let save_result = image.save(&filename);
    assert!(save_result.is_ok());
}

#[test]
fn test_convert_fail() {
    let path = Path::new("data/img1.jpg");
    let image = Image::open(path, LoadImageOptions::default());
    assert!(image.is_ok());
    let image = image.unwrap();
    let image = image.resize(ImageSize::new(100, 100), Interpolation::linear());
    let target_directory = std::env::temp_dir().join("test-images");
    std::fs::create_dir_all(&target_directory).unwrap();
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let filename = target_directory.join(format!("{}_100x100.bad_extension", now.as_secs()));
    let save_result = image.save(&filename);
    assert!(!save_result.is_ok());
}
