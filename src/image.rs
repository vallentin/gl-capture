pub mod prelude {
    pub use super::ImageData;
}

use crate::pixel::Pixel;

#[derive(Clone)]
pub struct ImageData<P>
where
    P: Pixel,
{
    pub size: (u32, u32),
    pub data: Vec<P>,
}

impl<P> ImageData<P>
where
    P: Pixel,
{
    pub fn new(size: (u32, u32)) -> Self {
        let pixel_count = (size.0 as usize) * (size.1 as usize);
        let mut data = vec![P::DEFAULT; pixel_count];
        data.shrink_to_fit();

        Self { size, data }
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.size = size;

        let pixel_count = (size.0 as usize) * (size.1 as usize);
        self.data.resize(pixel_count, P::DEFAULT);
    }

    pub(crate) fn flip_vertically(&mut self) {
        let (w, h) = self.size;
        for y in 0..(h / 2) {
            for x in 0..w {
                let top = (x + y * w) as usize;
                let bottom = (x + (h - y - 1) * w) as usize;
                self.data.swap(top, bottom);
            }
        }
    }
}
