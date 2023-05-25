pub mod prelude {
    pub use super::{Bgr, Bgra, Pixel, Rgb, Rgba};
}

use crate::CaptureFormat;

pub trait Pixel: Copy {
    const DEFAULT: Self;
    const FORMAT: CaptureFormat;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Rgb(pub [u8; 3]);

impl Rgb {
    pub(crate) const BLACK: Self = Self([0, 0, 0]);

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
}

impl Pixel for Rgb {
    const DEFAULT: Self = Self::BLACK;
    const FORMAT: CaptureFormat = CaptureFormat::Rgb;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Rgba(pub [u8; 4]);

impl Rgba {
    pub(crate) const BLACK: Self = Self([0, 0, 0, 255]);

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

impl Pixel for Rgba {
    const DEFAULT: Self = Self::BLACK;
    const FORMAT: CaptureFormat = CaptureFormat::Rgba;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Bgr(pub [u8; 3]);

impl Bgr {
    pub(crate) const BLACK: Self = Self([0, 0, 0]);

    #[inline]
    pub fn b(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn r(&self) -> u8 {
        self.0[2]
    }
}

impl Pixel for Bgr {
    const DEFAULT: Self = Self::BLACK;
    const FORMAT: CaptureFormat = CaptureFormat::Bgr;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct Bgra(pub [u8; 4]);

impl Bgra {
    pub(crate) const BLACK: Self = Self([0, 0, 0, 255]);

    #[inline]
    pub fn b(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn r(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub fn a(&self) -> u8 {
        self.0[3]
    }
}

impl Pixel for Bgra {
    const DEFAULT: Self = Self::BLACK;
    const FORMAT: CaptureFormat = CaptureFormat::Bgra;
}
