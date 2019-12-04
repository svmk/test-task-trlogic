use crate::ImageSize;
use libc::c_int;
use libc::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct IplImage {
    pub n_size: c_int,             /* < sizeof(IplImage) */
    pub id: c_int,                /* < version (=0)*/
    pub n_channels: c_int,         /* < Most of OpenCV functions support 1,2,3 or 4 channels */
    pub alpha_channel: c_int,      /* < Ignored by OpenCV */
    pub depth: c_int,             /* < Pixel depth in bits: IPL_DEPTH_8U, IPL_DEPTH_8S, IPL_DEPTH_16S,
                               IPL_DEPTH_32S, IPL_DEPTH_32F and IPL_DEPTH_64F are supported.  */
    pub color_model: [c_char; 4],     /* < Ignored by OpenCV */
    pub channel_seq: [c_char; 4],     /* < ditto */
    pub data_order: c_int,         /* < 0 - interleaved color channels, 1 - separate color channels.
                               cvCreateImage can only create interleaved images */
    pub origin: c_int,            /* < 0 - top-left origin,
                               1 - bottom-left origin (Windows bitmaps style).  */
    pub align: c_int,             /* < Alignment of image rows (4 or 8).
                               OpenCV ignores it and uses widthStep instead.    */
    pub width: c_int,             /* < Image width in pixels.                           */
    pub height: c_int,            /* < Image height in pixels.                          */
    pub roi: *const u8,           // struct _IplROI * /* < Image ROI. If NULL, the whole image is selected. */
    pub _ipl_image: *const u8,     // struct  *maskROI  /* < Must be NULL. */
    pub image_id: *const u8,       // void  *          /* < "           " */
    pub tile_info: *const u8,      // struct _IplTileInfo *  /* < "           " */
    pub image_size: c_int,         /* < Image data size in bytes
                               (==image->height*image->widthStep
                               in case of interleaved data)*/
    pub image_data: *const c_char,        /* < Pointer to aligned image data.         */
    pub width_step: c_int,         /* < Size of aligned image row in bytes.    */
    pub border_mode: [c_int; 4],     /* < Ignored by OpenCV.                     */
    pub border_const: [c_int; 4],    /* < Ditto.                                 */
    pub image_data_origin: *const c_char,  /* < Pointer to very origin of image data
                               (not necessarily aligned) -
                               needed for correct deallocation */
}

pub type CvArr = *const u8;

#[link(name = "opencv_imgcodecs")]
#[allow(non_snake_case)]
extern {
    pub fn cvLoadImage(filename: *const c_char, iscolor: c_int) -> *const IplImage;
    pub fn cvSaveImage(filename: *const c_char, image: CvArr, params: *const c_int) -> c_int;
    pub fn cvHaveImageReader(filename: *const c_char) -> c_int;
    pub fn cvHaveImageWriter(filename: *const c_char) -> c_int;
}


#[link(name = "opencv_core")]
#[allow(non_snake_case)]
extern {
    pub fn cvCreateImage(size: ImageSize, depth: c_int, channels: c_int) -> *const IplImage;
}


#[link(name = "opencv_imgproc")]
#[allow(non_snake_case)]
extern {
    pub fn cvResize(src: CvArr, dst: CvArr, interpolation: c_int);
}
