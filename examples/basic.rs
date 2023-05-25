#![forbid(elided_lifetimes_in_paths)]

use std::error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::slice;
use std::sync::mpsc::Receiver;

use gl_capture::{CaptureFormat, ImageData, Pixel, RgbImageData};
use glfw::{Context, Glfw, Key, OpenGlProfileHint, Window, WindowEvent, WindowHint, WindowMode};
use png::{BitDepth, ColorType, Encoder};

/// The `capture()` variants flip the resulting image
/// vertically, such that the pixel data layout is in
/// a more common format.
fn save_screenshot(size: (u32, u32)) {
    // Allocates an image on every call
    // let img = unsafe { gl_capture::capture() };

    // or cache `img` and use `capture_into()` to
    // avoid reallocating a new image every time
    let mut img = RgbImageData::new(size);
    unsafe {
        gl_capture::capture_into(&mut img);
    }
    // Remember to `img.resize()` when the framebuffer
    // has been resized

    let img = unsafe { gl_capture::capture() };
    if let Err(err) = save_png("screenshot.png", &img) {
        eprintln!("Error: {}", err);
    }
}

/// The `read_pixels` i.e. "raw" version essentially
/// just calls `gl::ReadPixels`.
fn save_screenshot_raw(size: (u32, u32)) {
    let format = CaptureFormat::Rgb;
    let mut data = format.allocate_pixel_data(size);

    unsafe {
        gl_capture::read_pixels((0, 0), size, format, &mut data);
    }

    if let Err(err) = save_png_raw("screenshot.png", format, size, &data) {
        eprintln!("Error: {}", err);
    }
}

fn main() {
    println!("Press F5 or Space to save a screenshot");

    let (mut glfw, mut wnd, events) = glfw_init();

    // Using scissor test to draw 4 colored rectangles
    unsafe {
        gl::Enable(gl::SCISSOR_TEST);
    }

    'main: loop {
        glfw.poll_events();

        for (_timestamp, evt) in glfw::flush_messages(&events) {
            match evt {
                WindowEvent::Key(Key::F5 | Key::Space, _, glfw::Action::Press, _) => {
                    let (w, h) = wnd.get_framebuffer_size();
                    let size = (w as u32, h as u32);
                    save_screenshot(size);
                    save_screenshot_raw(size);
                }
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    break 'main;
                }
                _ => {}
            }
        }

        let (w, h) = wnd.get_framebuffer_size();
        let (hw, hh) = (w / 2, h / 2);

        unsafe {
            gl::Viewport(0, 0, w, h);

            draw_rect((0, 0), (hw, hh), (1.0, 0.0, 0.0));
            draw_rect((hw, 0), (hw, hh), (0.0, 1.0, 0.0));
            draw_rect((0, hh), (hw, hh), (0.0, 0.0, 1.0));
            draw_rect((hw, hh), (hw, hh), (0.0, 0.0, 0.0));

            draw_rect((0, hh / 2), (hw / 5, hh), (1.0, 0.0, 1.0));
            draw_rect((hw / 2, 0), (hw, hh / 5), (1.0, 1.0, 0.0));

            draw_rect((hw / 2, hh / 2), (hw, hh), (1.0, 1.0, 1.0));
        }

        wnd.swap_buffers();
    }
}

fn save_png<P>(path: impl AsRef<Path>, img: &ImageData<P>) -> Result<(), Box<dyn error::Error>>
where
    P: Pixel,
{
    let pixels =
        unsafe { slice::from_raw_parts(img.data.as_ptr() as *const u8, img.data.len() * 4) };
    save_png_raw(path, P::FORMAT, img.size, pixels)
}

fn save_png_raw(
    path: impl AsRef<Path>,
    format: CaptureFormat,
    size: (u32, u32),
    data: &[u8],
) -> Result<(), Box<dyn error::Error>> {
    let path = path.as_ref();

    println!("Saving `{}`", path.display());

    let f = File::create(path)?;
    let w = BufWriter::new(f);
    let mut encoder = Encoder::new(w, size.0, size.1);
    encoder.set_color(match format {
        CaptureFormat::Rgba => ColorType::Rgba,
        CaptureFormat::Rgb => ColorType::Rgb,
        _ => unimplemented!(),
    });
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(data)?;

    Ok(())
}

unsafe fn draw_rect((x, y): (i32, i32), (w, h): (i32, i32), (r, g, b): (f32, f32, f32)) {
    // DO NOT USE IN PRODUCTION
    // ONLY USING SCISSOR AND CLEAR AS IT PROVIDES
    // A SIMPLE WAY TO DEMONSTRATE A WORKING
    // SCREENSHOT CAPTURING

    gl::Scissor(x, y, w, h);
    gl::ClearColor(r, g, b, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

fn glfw_init() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    let mut glfw = glfw::init(Some(glfw::Callback {
        f: |err, desc, _| panic!("glfw error [{}]: {}", err, desc),
        data: (),
    }))
    .expect("unable to initialize glfw");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::Visible(false));

    let (mut wnd, events) = glfw
        .create_window(856, 482, env!("CARGO_PKG_NAME"), WindowMode::Windowed)
        .unwrap();

    wnd.set_key_polling(true);
    wnd.set_framebuffer_size_polling(true);
    wnd.set_close_polling(true);

    wnd.make_current();

    gl::load_with(|symbol| wnd.get_proc_address(symbol) as *const _);

    wnd.show();

    (glfw, wnd, events)
}
