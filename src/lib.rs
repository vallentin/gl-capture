#![forbid(elided_lifetimes_in_paths)]
#![cfg_attr(debug_assertions, allow(dead_code, unreachable_code))]

pub use crate::image::prelude::*;
pub use crate::pixel::prelude::*;

mod image;
mod pixel;

use std::ffi::c_void;
use std::mem;

use gl::types::GLenum;

pub type RgbaImageData = ImageData<Rgba>;
pub type RgbImageData = ImageData<Rgb>;

pub type BgraImageData = ImageData<Bgra>;
pub type BgrImageData = ImageData<Bgr>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CaptureFormat {
    Rgba,
    Rgb,
    Bgra,
    Bgr,
}

impl CaptureFormat {
    pub fn allocate_pixel_data(self, (w, h): (u32, u32)) -> Vec<u8> {
        let size = (w as usize) * (h as usize) * self.channel_count();
        vec![0; size]
    }

    pub const fn to_gl_format(self) -> GLenum {
        match self {
            Self::Rgba => gl::RGBA,
            Self::Rgb => gl::RGB,
            Self::Bgra => gl::BGRA,
            Self::Bgr => gl::BGR,
        }
    }

    pub const fn channel_count(self) -> usize {
        match self {
            Self::Rgba | Self::Bgra => 4,
            Self::Rgb | Self::Bgr => 3,
        }
    }

    /// Returns the pixel byte size of this capture format.
    pub const fn pixel_size(self) -> usize {
        match self {
            Self::Rgba | Self::Bgra => mem::size_of::<[u8; 4]>(),
            Self::Rgb | Self::Bgr => mem::size_of::<[u8; 3]>(),
        }
    }
}

pub unsafe fn capture() -> RgbImageData {
    let mut viewport = [0; 4];
    gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr());

    let size = (viewport[2] as u32, viewport[3] as u32);
    let mut img = RgbImageData::new(size);
    capture_into(&mut img);
    img
}

pub unsafe fn capture_into<P>(img: &mut ImageData<P>)
where
    P: Pixel,
{
    read_pixels_ptr(
        (0, 0),
        img.size,
        P::FORMAT,
        img.data.as_mut_ptr() as *mut c_void,
    );
    img.flip_vertically();
}

/// Panics if the length of pixel `data`, is not
/// at least `w * h * format.`[`byte_size()`](CaptureFormat::byte_size)
pub unsafe fn read_pixels(
    (x, y): (u32, u32),
    (w, h): (u32, u32),
    format: CaptureFormat,
    data: &mut [u8],
) {
    assert!(w <= (i32::MAX as u32));
    assert!(h <= (i32::MAX as u32));

    let min_len = (w as usize) * (h as usize) * format.channel_count();
    assert!(data.len() <= min_len);

    read_pixels_ptr((x, y), (w, h), format, data.as_mut_ptr() as *mut c_void);
}

pub unsafe fn read_pixels_ptr(
    (x, y): (u32, u32),
    (w, h): (u32, u32),
    format: CaptureFormat,
    data: *mut c_void,
) {
    gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
    gl::ReadPixels(
        x as i32,
        y as i32,
        w as i32,
        h as i32,
        format.to_gl_format(),
        gl::UNSIGNED_BYTE,
        data,
    );
}
