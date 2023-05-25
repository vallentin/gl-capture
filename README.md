# gl-capture

[![Latest Version](https://img.shields.io/crates/v/gl-capture.svg)](https://crates.io/crates/gl-capture)
[![Docs](https://docs.rs/gl-capture/badge.svg)](https://docs.rs/gl-capture)
[![License](https://img.shields.io/github/license/vallentin/gl-capture.svg)](https://github.com/vallentin/gl-capture)

Library for capturing screenshots in OpenGL.

See [examples/basic.rs] for a complete example.

```rust
let img = unsafe { gl_capture::capture() };
// img.size: (u32, u32)
// img.data: Vec<capture_gl::Rgb>
// Now use e.g. `png` or `image` crate to save the image data to a file
```

Alternatively, use [`capture_into()`] to reuse the same image data, instead
of reallocating on every call.

```rust
let mut img = gl_capture::RgbImageData::new(size);
unsafe {
    gl_capture::capture_into(&mut img);
}
// img.size: (u32, u32)
// img.data: Vec<capture_gl::Rgb>
```

Also supports other formats, e.g. [`RgbaImageData`], [`BgrImageData`], [`BgraImageData`].

When manually using `gl::ReadPixels()`, instead it is also possible to use
[`read_pixels()`] or [`read_pixels_ptr()`], which performs some additional checks
and setup.

```rust
let format = gl_capture::CaptureFormat::Rgb;
let mut data = format.allocate_pixel_data(size);

unsafe {
    gl_capture::read_pixels((0, 0), size, format, &mut data);
}
```

[examples/basic.rs]: https://github.com/vallentin/gl-capture/blob/master/examples/basic.rs

[`capture_into()`]: https://docs.rs/gl-capture/*/gl_capture/fn.capture_into.html

[`RgbaImageData`]: https://docs.rs/gl-capture/*/gl_capture/type.RgbaImageData.html
[`BgrImageData`]: https://docs.rs/gl-capture/*/gl_capture/type.BgrImageData.html
[`BgraImageData`]: https://docs.rs/gl-capture/*/gl_capture/type.BgraImageData.html

[`read_pixels()`]: https://docs.rs/gl-capture/*/gl_capture/fn.read_pixels.html
[`read_pixels_ptr()`]: https://docs.rs/gl-capture/*/gl_capture/fn.read_pixels_ptr.html
