#![forbid(elided_lifetimes_in_paths)]
#![cfg_attr(debug_assertions, allow(dead_code, unreachable_code))]

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Rgba(pub [u8; 4]);

impl Rgba {
    const BLACK: Self = Self([0, 0, 0, 255]);

    #[inline]
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub fn a(&self) -> u8 {
        self.0[3]
    }
}

#[derive(Clone)]
pub struct RgbaImageData {
    pub size: (u32, u32),
    pub data: Vec<Rgba>,
}

impl RgbaImageData {
    pub fn new(size: (u32, u32)) -> Self {
        let pixel_count = (size.0 as usize) * (size.1 as usize);
        let mut data = vec![Rgba::BLACK; pixel_count];
        data.shrink_to_fit();

        Self { size, data }
    }
}

pub unsafe fn capture<'a>() -> RgbaImageData {
    let mut viewport = [0; 4];
    gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr());

    let size = (viewport[2] as u32, viewport[3] as u32);
    let mut img = RgbaImageData::new(size);
    capture_into(&mut img);
    img
}

pub unsafe fn capture_into<'a>(img: &mut RgbaImageData) {
    assert!(img.size.0 <= (i32::MAX as u32));
    assert!(img.size.1 <= (i32::MAX as u32));

    gl::PixelStorei(gl::PACK_ALIGNMENT, 1);
    gl::ReadPixels(
        0,
        0,
        img.size.0 as i32,
        img.size.1 as i32,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        img.data.as_mut_ptr() as *mut _,
    );
}
