use std::io::{Error, ErrorKind};
use std::path::Path;

use bindings::ffi;
use bindings::{AsNative, FromNative, CString};
use colors::Color;
use console::{BackgroundFlag, Console};

pub struct Image {
    tcod_image: ffi::TCOD_image_t,
    width: i32,
    height: i32,
}

impl AsNative<ffi::TCOD_image_t> for Image {
    unsafe fn as_native(&self) -> &ffi::TCOD_image_t {
        &self.tcod_image
    }
}

impl FromNative<ffi::TCOD_image_t> for Image {
    unsafe fn from_native(image: ffi::TCOD_image_t) -> Image {
        let (width, height) = get_image_size(image);
        assert!(width != 0);
        Image { tcod_image: image, width: width, height: height }
    }
}

#[inline]
unsafe fn get_image_size(tcod_image: ffi::TCOD_image_t) -> (i32, i32) {
    let (mut width, mut height) = (0, 0);
    ffi::TCOD_image_get_size(tcod_image, &mut width, &mut height);
    (width, height)
}

impl Image {
    pub fn new(width: i32, height: i32) -> Image {
        unsafe {
            Image {
                tcod_image: ffi::TCOD_image_new(width, height),
                width: width,
                height: height,
            }
        }
    }

    pub fn from_file<T>(path: T) -> Result<Image, Error> where T: AsRef<Path> {
        let path_string = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            let tcod_image = ffi::TCOD_image_load(path_string.as_ptr());
            let (width, height) = get_image_size(tcod_image);

            if width == 0 {
                Err(Error::new(ErrorKind::InvalidInput, "The provided image format is not supported by libtcod"))
            } else {
                Ok(Image { tcod_image: tcod_image, width: width, height: height })
            }
        }
    }

    pub fn from_console<T>(console: &T) -> Image where T: Console {
        unsafe {
            let tcod_image = ffi::TCOD_image_from_console(*console.as_native());
            let (width, height) = get_image_size(tcod_image);
            Image {
                tcod_image: tcod_image,
                width: width,
                height: height
            }
        }
    }

    pub fn refresh_console<T>(&mut self, console: &T) where T: Console {
        assert!(
            {
                let img = Image::from_console(console);
                self.width == img.width && self.height == img.height
            },

            "libtcod only supports console refreshing with consoles of equivalent sizes"
        );

        unsafe {
            ffi::TCOD_image_refresh_console(self.tcod_image, *console.as_native());
        }
    }

    pub fn save<T>(&self, path: T) where T: AsRef<Path> {
        let path_string = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            ffi::TCOD_image_save(self.tcod_image, path_string.as_ptr());
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            FromNative::from_native(ffi::TCOD_image_get_pixel(self.tcod_image, x, y))
        }
    }

    pub fn get_alpha(&self, x: i32, y: i32) -> i32 {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            ffi::TCOD_image_get_alpha(self.tcod_image, x, y)
        }
    }

    pub fn is_pixel_transparent(&self, x: i32, y: i32) -> bool {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            ffi::TCOD_image_is_pixel_transparent(self.tcod_image, x, y) != 0
        }
    }

    pub fn get_mipmap_pixel(&self, (x0, y0): (f32, f32), (x1, y1): (f32, f32)) -> Color {
        assert!(x0 >= 0.0 && y0 >= 0.0 &&
                x0 < x1 && y0 < y1 &&
                x1 < self.width as f32 && y1 < self.height as f32);
        unsafe {
            FromNative::from_native(ffi::TCOD_image_get_mipmap_pixel(self.tcod_image, x0, y0, x1, y1))
        }
    }

    pub fn set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_image_set_key_color(self.tcod_image, *color.as_native());
        }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_image_clear(self.tcod_image, *color.as_native());
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            ffi::TCOD_image_put_pixel(self.tcod_image, x, y, *color.as_native());
        }
    }

    pub fn scale(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::TCOD_image_scale(self.tcod_image, width, height);
        }
        self.width = width;
        self.height = height;
    }

    pub fn hflip(&mut self) {
        unsafe {
            ffi::TCOD_image_hflip(self.tcod_image);
        }
    }

    pub fn vflip(&mut self) {
        unsafe {
            ffi::TCOD_image_vflip(self.tcod_image);
        }
    }

    pub fn rotate90(&mut self, num_rotations: i32) {
        let (width, height) = unsafe {
            ffi::TCOD_image_rotate90(self.tcod_image, num_rotations);
            get_image_size(self.tcod_image)
        };
        self.width = width;
        self.height = height;
    }

    pub fn invert(&mut self) {
        unsafe {
            ffi::TCOD_image_invert(self.tcod_image);
        }
    }
}

pub fn blit_rect<T>(src: &Image, (width, height): (i32, i32),
                    dst: &mut T, (x, y): (i32, i32), flag: BackgroundFlag) where T: Console {
    assert!(width >= -1 && height >= -1 && width <= src.width && height <= src.height);
    assert!(x >= 0 && y >= 0 && x < dst.width() && y < dst.height());
    unsafe {
        ffi::TCOD_image_blit_rect(src.tcod_image, *dst.as_native(), x, y, width, height, flag as u32);
    }
}

pub fn blit<T>(src: &Image, (scale_x, scale_y): (f32, f32), angle: f32,
               dst: &mut T, (x, y): (f32, f32),  flag: BackgroundFlag) where T: Console {
    assert!(scale_x > 0.0 && scale_y > 0.0);
    assert!(x >= 0.0 && y >= 0.0 && x < dst.width() as f32 && y < dst.height() as f32);
    unsafe {
        ffi::TCOD_image_blit(src.tcod_image, *dst.as_native(), x, y, flag as u32, scale_x, scale_y, angle);
    }
}

pub fn blit_2x<T>(src: &Image, (src_x, src_y): (i32, i32), (width, height): (i32, i32),
                  dst: &mut T,  (dst_x, dst_y): (i32, i32)) where T: Console {
    assert!(width >= -1 && height >= -1 && width <= src.width && height <= src.height);
    assert!(src_x >= 0 && src_y >= 0 && src_x < src.width && src_y < src.height);
    assert!(dst_x >= 0 && dst_y >= 0 && dst_x < dst.width() && dst_y < dst.height());
    unsafe {
        ffi::TCOD_image_blit_2x(src.tcod_image, *dst.as_native(), dst_x, dst_y, src_x, src_y, width, height);
    }
}
