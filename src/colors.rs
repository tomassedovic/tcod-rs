#![allow(non_upper_case_globals)]

use bindings::ffi;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_tcod_color_t(tcod_color_t: ffi::TCOD_color_t) -> Color {
        unsafe {
            ::std::mem::transmute(tcod_color_t)
        }
    }

    pub fn to_color_t(self) -> ffi::TCOD_color_t {
        unsafe {
            ::std::mem::transmute(self)
        }
    }

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn new_from_hsv(h: f32, s: f32, v: f32) -> Color {
        let mut tcod_c = Color{r: 0, g: 0, b: 0}.to_color_t();
        unsafe {
            ffi::TCOD_color_set_HSV(&mut tcod_c, h, s, v)
        }
        Color::from_tcod_color_t(tcod_c)
    }

    pub fn multiply(self, other: Color) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_color_multiply(self.to_color_t(), other.to_color_t()))
        }
    }

    pub fn multiply_scalar(self, val: f32) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_color_multiply_scalar(self.to_color_t(), val))
        }
    }

    pub fn add(self, other: Color) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_color_add(self.to_color_t(), other.to_color_t()))
        }
    }

    pub fn subtract(self, other: Color) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_color_subtract(self.to_color_t(), other.to_color_t()))
        }
    }

    pub fn lerp(self, to: Color, coefficient: f32) -> Color {
        unsafe {
            Color::from_tcod_color_t(ffi::TCOD_color_lerp(self.to_color_t(),
                                                          to.to_color_t(),
                                                          coefficient))
        }
    }

    pub fn hsv(self) -> (f32, f32, f32) {
        let mut h: f32 = 0.0;
        let mut s: f32 = 0.0;
        let mut v: f32 = 0.0;
        unsafe {
            ffi::TCOD_color_get_HSV(self.to_color_t(), &mut h, &mut s, &mut v)
        }
        (h, s, v)
    }

    pub fn shift_hue(self, shift: f32) -> Color {
        let mut c = self.to_color_t();
        unsafe {
            ffi::TCOD_color_shift_hue(&mut c, shift);
        }
        Color::from_tcod_color_t(c)
    }

    pub fn scale_hsv(self, scale: f32, value: f32) -> Color {
        let mut c = self.to_color_t();
        unsafe {
            ffi::TCOD_color_scale_HSV(&mut c, scale, value);
        }
        Color::from_tcod_color_t(c)
    }
}


// NOTE; colour names and values copied from:
// tcod-sys/libtcod/include/libtcod_int.h
//
// We cannot return statics exported by the DLL here because they have a
// different type (TCOD_color_t) and we cannot call `transmute` to convert
// them to `Color`.
pub const black: Color = Color{r: 0, g: 0, b: 0};
pub const darkest_grey: Color = Color{r: 31, g: 31, b: 31};
pub const darker_grey: Color = Color{r: 63, g: 63, b: 63};
pub const dark_grey: Color = Color{r: 95, g: 95, b: 95};
pub const grey: Color = Color{r: 127, g: 127, b: 127};
pub const light_grey: Color = Color{r: 159, g: 159, b: 159};
pub const lighter_grey: Color = Color{r: 191, g: 191, b: 191};
pub const lightest_grey: Color = Color{r: 223, g: 223, b: 223};
pub const white: Color = Color{r: 255, g: 255, b: 255};
pub const darkest_sepia: Color = Color{r: 31, g: 24, b: 15};
pub const darker_sepia: Color = Color{r: 63, g: 50, b: 31};
pub const dark_sepia: Color = Color{r: 94, g: 75, b: 47};
pub const sepia: Color = Color{r: 127, g: 101, b: 63};
pub const light_sepia: Color = Color{r: 158, g: 134, b: 100};
pub const lighter_sepia: Color = Color{r: 191, g: 171, b: 143};
pub const lightest_sepia: Color = Color{r: 222, g: 211, b: 195};
pub const desaturated_red: Color = Color{r: 127, g: 63, b: 63};
pub const desaturated_flame: Color = Color{r: 127, g: 79, b: 63};
pub const desaturated_orange: Color = Color{r: 127, g: 95, b: 63};
pub const desaturated_amber: Color = Color{r: 127, g: 111, b: 63};
pub const desaturated_yellow: Color = Color{r: 127, g: 127, b: 63};
pub const desaturated_lime: Color = Color{r: 111, g: 127, b: 63};
pub const desaturated_chartreuse: Color = Color{r: 95, g: 127, b: 63};
pub const desaturated_green: Color = Color{r: 63, g: 127, b: 63};
pub const desaturated_sea: Color = Color{r: 63, g: 127, b: 95};
pub const desaturated_turquoise: Color = Color{r: 63, g: 127, b: 111};
pub const desaturated_cyan: Color = Color{r: 63, g: 127, b: 127};
pub const desaturated_sky: Color = Color{r: 63, g: 111, b: 127};
pub const desaturated_azure: Color = Color{r: 63, g: 95, b: 127};
pub const desaturated_blue: Color = Color{r: 63, g: 63, b: 127};
pub const desaturated_han: Color = Color{r: 79, g: 63, b: 127};
pub const desaturated_violet: Color = Color{r: 95, g: 63, b: 127};
pub const desaturated_purple: Color = Color{r: 111, g: 63, b: 127};
pub const desaturated_fuchsia: Color = Color{r: 127, g: 63, b: 127};
pub const desaturated_magenta: Color = Color{r: 127, g: 63, b: 111};
pub const desaturated_pink: Color = Color{r: 127, g: 63, b: 95};
pub const desaturated_crimson: Color = Color{r: 127, g: 63, b: 79};
pub const lightest_red: Color = Color{r: 255, g: 191, b: 191};
pub const lightest_flame: Color = Color{r: 255, g: 207, b: 191};
pub const lightest_orange: Color = Color{r: 255, g: 223, b: 191};
pub const lightest_amber: Color = Color{r: 255, g: 239, b: 191};
pub const lightest_yellow: Color = Color{r: 255, g: 255, b: 191};
pub const lightest_lime: Color = Color{r: 239, g: 255, b: 191};
pub const lightest_chartreuse: Color = Color{r: 223, g: 255, b: 191};
pub const lightest_green: Color = Color{r: 191, g: 255, b: 191};
pub const lightest_sea: Color = Color{r: 191, g: 255, b: 223};
pub const lightest_turquoise: Color = Color{r: 191, g: 255, b: 239};
pub const lightest_cyan: Color = Color{r: 191, g: 255, b: 255};
pub const lightest_sky: Color = Color{r: 191, g: 239, b: 255};
pub const lightest_azure: Color = Color{r: 191, g: 223, b: 255};
pub const lightest_blue: Color = Color{r: 191, g: 191, b: 255};
pub const lightest_han: Color = Color{r: 207, g: 191, b: 255};
pub const lightest_violet: Color = Color{r: 223, g: 191, b: 255};
pub const lightest_purple: Color = Color{r: 239, g: 191, b: 255};
pub const lightest_fuchsia: Color = Color{r: 255, g: 191, b: 255};
pub const lightest_magenta: Color = Color{r: 255, g: 191, b: 239};
pub const lightest_pink: Color = Color{r: 255, g: 191, b: 223};
pub const lightest_crimson: Color = Color{r: 255, g: 191, b: 207};
pub const lighter_red: Color = Color{r: 255, g: 127, b: 127};
pub const lighter_flame: Color = Color{r: 255, g: 159, b: 127};
pub const lighter_orange: Color = Color{r: 255, g: 191, b: 127};
pub const lighter_amber: Color = Color{r: 255, g: 223, b: 127};
pub const lighter_yellow: Color = Color{r: 255, g: 255, b: 127};
pub const lighter_lime: Color = Color{r: 223, g: 255, b: 127};
pub const lighter_chartreuse: Color = Color{r: 191, g: 255, b: 127};
pub const lighter_green: Color = Color{r: 127, g: 255, b: 127};
pub const lighter_sea: Color = Color{r: 127, g: 255, b: 191};
pub const lighter_turquoise: Color = Color{r: 127, g: 255, b: 223};
pub const lighter_cyan: Color = Color{r: 127, g: 255, b: 255};
pub const lighter_sky: Color = Color{r: 127, g: 223, b: 255};
pub const lighter_azure: Color = Color{r: 127, g: 191, b: 255};
pub const lighter_blue: Color = Color{r: 127, g: 127, b: 255};
pub const lighter_han: Color = Color{r: 159, g: 127, b: 255};
pub const lighter_violet: Color = Color{r: 191, g: 127, b: 255};
pub const lighter_purple: Color = Color{r: 223, g: 127, b: 255};
pub const lighter_fuchsia: Color = Color{r: 255, g: 127, b: 255};
pub const lighter_magenta: Color = Color{r: 255, g: 127, b: 223};
pub const lighter_pink: Color = Color{r: 255, g: 127, b: 191};
pub const lighter_crimson: Color = Color{r: 255, g: 127, b: 159};
pub const light_red: Color = Color{r: 255, g: 63, b: 63};
pub const light_flame: Color = Color{r: 255, g: 111, b: 63};
pub const light_orange: Color = Color{r: 255, g: 159, b: 63};
pub const light_amber: Color = Color{r: 255, g: 207, b: 63};
pub const light_yellow: Color = Color{r: 255, g: 255, b: 63};
pub const light_lime: Color = Color{r: 207, g: 255, b: 63};
pub const light_chartreuse: Color = Color{r: 159, g: 255, b: 63};
pub const light_green: Color = Color{r: 63, g: 255, b: 63};
pub const light_sea: Color = Color{r: 63, g: 255, b: 159};
pub const light_turquoise: Color = Color{r: 63, g: 255, b: 207};
pub const light_cyan: Color = Color{r: 63, g: 255, b: 255};
pub const light_sky: Color = Color{r: 63, g: 207, b: 255};
pub const light_azure: Color = Color{r: 63, g: 159, b: 255};
pub const light_blue: Color = Color{r: 63, g: 63, b: 255};
pub const light_han: Color = Color{r: 111, g: 63, b: 255};
pub const light_violet: Color = Color{r: 159, g: 63, b: 255};
pub const light_purple: Color = Color{r: 207, g: 63, b: 255};
pub const light_fuchsia: Color = Color{r: 255, g: 63, b: 255};
pub const light_magenta: Color = Color{r: 255, g: 63, b: 207};
pub const light_pink: Color = Color{r: 255, g: 63, b: 159};
pub const light_crimson: Color = Color{r: 255, g: 63, b: 111};
pub const red: Color = Color{r: 255, g: 0, b: 0};
pub const flame: Color = Color{r: 255, g: 63, b: 0};
pub const orange: Color = Color{r: 255, g: 127, b: 0};
pub const amber: Color = Color{r: 255, g: 191, b: 0};
pub const yellow: Color = Color{r: 255, g: 255, b: 0};
pub const lime: Color = Color{r: 191, g: 255, b: 0};
pub const chartreuse: Color = Color{r: 127, g: 255, b: 0};
pub const green: Color = Color{r: 0, g: 255, b: 0};
pub const sea: Color = Color{r: 0, g: 255, b: 127};
pub const turquoise: Color = Color{r: 0, g: 255, b: 191};
pub const cyan: Color = Color{r: 0, g: 255, b: 255};
pub const sky: Color = Color{r: 0, g: 191, b: 255};
pub const azure: Color = Color{r: 0, g: 127, b: 255};
pub const blue: Color = Color{r: 0, g: 0, b: 255};
pub const han: Color = Color{r: 63, g: 0, b: 255};
pub const violet: Color = Color{r: 127, g: 0, b: 255};
pub const purple: Color = Color{r: 191, g: 0, b: 255};
pub const fuchsia: Color = Color{r: 255, g: 0, b: 255};
pub const magenta: Color = Color{r: 255, g: 0, b: 191};
pub const pink: Color = Color{r: 255, g: 0, b: 127};
pub const crimson: Color = Color{r: 255, g: 0, b: 63};
pub const dark_red: Color = Color{r: 191, g: 0, b: 0};
pub const dark_flame: Color = Color{r: 191, g: 47, b: 0};
pub const dark_orange: Color = Color{r: 191, g: 95, b: 0};
pub const dark_amber: Color = Color{r: 191, g: 143, b: 0};
pub const dark_yellow: Color = Color{r: 191, g: 191, b: 0};
pub const dark_lime: Color = Color{r: 143, g: 191, b: 0};
pub const dark_chartreuse: Color = Color{r: 95, g: 191, b: 0};
pub const dark_green: Color = Color{r: 0, g: 191, b: 0};
pub const dark_sea: Color = Color{r: 0, g: 191, b: 95};
pub const dark_turquoise: Color = Color{r: 0, g: 191, b: 143};
pub const dark_cyan: Color = Color{r: 0, g: 191, b: 191};
pub const dark_sky: Color = Color{r: 0, g: 143, b: 191};
pub const dark_azure: Color = Color{r: 0, g: 95, b: 191};
pub const dark_blue: Color = Color{r: 0, g: 0, b: 191};
pub const dark_han: Color = Color{r: 47, g: 0, b: 191};
pub const dark_violet: Color = Color{r: 95, g: 0, b: 191};
pub const dark_purple: Color = Color{r: 143, g: 0, b: 191};
pub const dark_fuchsia: Color = Color{r: 191, g: 0, b: 191};
pub const dark_magenta: Color = Color{r: 191, g: 0, b: 143};
pub const dark_pink: Color = Color{r: 191, g: 0, b: 95};
pub const dark_crimson: Color = Color{r: 191, g: 0, b: 47};
pub const darker_red: Color = Color{r: 127, g: 0, b: 0};
pub const darker_flame: Color = Color{r: 127, g: 31, b: 0};
pub const darker_orange: Color = Color{r: 127, g: 63, b: 0};
pub const darker_amber: Color = Color{r: 127, g: 95, b: 0};
pub const darker_yellow: Color = Color{r: 127, g: 127, b: 0};
pub const darker_lime: Color = Color{r: 95, g: 127, b: 0};
pub const darker_chartreuse: Color = Color{r: 63, g: 127, b: 0};
pub const darker_green: Color = Color{r: 0, g: 127, b: 0};
pub const darker_sea: Color = Color{r: 0, g: 127, b: 63};
pub const darker_turquoise: Color = Color{r: 0, g: 127, b: 95};
pub const darker_cyan: Color = Color{r: 0, g: 127, b: 127};
pub const darker_sky: Color = Color{r: 0, g: 95, b: 127};
pub const darker_azure: Color = Color{r: 0, g: 63, b: 127};
pub const darker_blue: Color = Color{r: 0, g: 0, b: 127};
pub const darker_han: Color = Color{r: 31, g: 0, b: 127};
pub const darker_violet: Color = Color{r: 63, g: 0, b: 127};
pub const darker_purple: Color = Color{r: 95, g: 0, b: 127};
pub const darker_fuchsia: Color = Color{r: 127, g: 0, b: 127};
pub const darker_magenta: Color = Color{r: 127, g: 0, b: 95};
pub const darker_pink: Color = Color{r: 127, g: 0, b: 63};
pub const darker_crimson: Color = Color{r: 127, g: 0, b: 31};
pub const darkest_red: Color = Color{r: 63, g: 0, b: 0};
pub const darkest_flame: Color = Color{r: 63, g: 15, b: 0};
pub const darkest_orange: Color = Color{r: 63, g: 31, b: 0};
pub const darkest_amber: Color = Color{r: 63, g: 47, b: 0};
pub const darkest_yellow: Color = Color{r: 63, g: 63, b: 0};
pub const darkest_lime: Color = Color{r: 47, g: 63, b: 0};
pub const darkest_chartreuse: Color = Color{r: 31, g: 63, b: 0};
pub const darkest_green: Color = Color{r: 0, g: 63, b: 0};
pub const darkest_sea: Color = Color{r: 0, g: 63, b: 31};
pub const darkest_turquoise: Color = Color{r: 0, g: 63, b: 47};
pub const darkest_cyan: Color = Color{r: 0, g: 63, b: 63};
pub const darkest_sky: Color = Color{r: 0, g: 47, b: 63};
pub const darkest_azure: Color = Color{r: 0, g: 31, b: 63};
pub const darkest_blue: Color = Color{r: 0, g: 0, b: 63};
pub const darkest_han: Color = Color{r: 15, g: 0, b: 63};
pub const darkest_violet: Color = Color{r: 31, g: 0, b: 63};
pub const darkest_purple: Color = Color{r: 47, g: 0, b: 63};
pub const darkest_fuchsia: Color = Color{r: 63, g: 0, b: 63};
pub const darkest_magenta: Color = Color{r: 63, g: 0, b: 47};
pub const darkest_pink: Color = Color{r: 63, g: 0, b: 31};
pub const darkest_crimson: Color = Color{r: 63, g: 0, b: 15};
pub const brass: Color = Color{r: 191, g: 151, b: 96};
pub const copper: Color = Color{r: 197, g: 136, b: 124};
pub const gold: Color = Color{r: 229, g: 191, b: 0};
pub const silver: Color = Color{r: 203, g: 203, b: 203};
pub const celadon: Color = Color{r: 172, g: 255, b: 175};
pub const peach: Color = Color{r: 255, g: 159, b: 127};

