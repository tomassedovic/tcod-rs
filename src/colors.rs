use bindings::{AsNative, FromNative, ffi};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl AsNative<ffi::TCOD_color_t> for Color {
    unsafe fn as_native(&self) -> &ffi::TCOD_color_t {
        ::std::mem::transmute(self)
    }
}

impl FromNative<ffi::TCOD_color_t> for Color {
    unsafe fn from_native(input: ffi::TCOD_color_t) -> Color {
        ::std::mem::transmute(input)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn new_from_hsv(h: f32, s: f32, v: f32) -> Color {
        unsafe {
            let mut tcod_c = *Color{r: 0, g: 0, b: 0}.as_native();
            ffi::TCOD_color_set_HSV(&mut tcod_c, h, s, v);
            FromNative::from_native(tcod_c)
        }
    }

    pub fn multiply(self, other: Color) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_color_multiply(*self.as_native(), *other.as_native()))
        }
    }

    pub fn multiply_scalar(self, val: f32) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_color_multiply_scalar(*self.as_native(), val))
        }
    }

    pub fn add(self, other: Color) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_color_add(*self.as_native(), *other.as_native()))
        }
    }

    pub fn subtract(self, other: Color) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_color_subtract(*self.as_native(), *other.as_native()))
        }
    }

    pub fn lerp(self, to: Color, coefficient: f32) -> Color {
        unsafe {
            FromNative::from_native(ffi::TCOD_color_lerp(*self.as_native(),
                                                         *to.as_native(),
                                                         coefficient))
        }
    }

    pub fn hsv(self) -> (f32, f32, f32) {
        let mut h: f32 = 0.0;
        let mut s: f32 = 0.0;
        let mut v: f32 = 0.0;
        unsafe {
            ffi::TCOD_color_get_HSV(*self.as_native(), &mut h, &mut s, &mut v)
        }
        (h, s, v)
    }

    pub fn shift_hue(self, shift: f32) -> Color {
        unsafe {
            let mut c = *self.as_native();
            ffi::TCOD_color_shift_hue(&mut c, shift);
            FromNative::from_native(c)
        }
    }

    pub fn scale_hsv(self, scale: f32, value: f32) -> Color {
        unsafe {
            let mut c = *self.as_native();
            ffi::TCOD_color_scale_HSV(&mut c, scale, value);
            FromNative::from_native(c)
        }
    }
}


// NOTE; colour names and values copied from:
// tcod-sys/libtcod/include/libtcod_int.h
//
// We cannot return statics exported by the DLL here because they have a
// different type (TCOD_color_t) and we cannot call `transmute` to convert
// them to `Color`.
pub const BLACK: Color = Color{r: 0, g: 0, b: 0};
pub const DARKEST_GREY: Color = Color{r: 31, g: 31, b: 31};
pub const DARKER_GREY: Color = Color{r: 63, g: 63, b: 63};
pub const DARK_GREY: Color = Color{r: 95, g: 95, b: 95};
pub const GREY: Color = Color{r: 127, g: 127, b: 127};
pub const LIGHT_GREY: Color = Color{r: 159, g: 159, b: 159};
pub const LIGHTER_GREY: Color = Color{r: 191, g: 191, b: 191};
pub const LIGHTEST_GREY: Color = Color{r: 223, g: 223, b: 223};
pub const WHITE: Color = Color{r: 255, g: 255, b: 255};
pub const DARKEST_SEPIA: Color = Color{r: 31, g: 24, b: 15};
pub const DARKER_SEPIA: Color = Color{r: 63, g: 50, b: 31};
pub const DARK_SEPIA: Color = Color{r: 94, g: 75, b: 47};
pub const SEPIA: Color = Color{r: 127, g: 101, b: 63};
pub const LIGHT_SEPIA: Color = Color{r: 158, g: 134, b: 100};
pub const LIGHTER_SEPIA: Color = Color{r: 191, g: 171, b: 143};
pub const LIGHTEST_SEPIA: Color = Color{r: 222, g: 211, b: 195};
pub const DESATURATED_RED: Color = Color{r: 127, g: 63, b: 63};
pub const DESATURATED_FLAME: Color = Color{r: 127, g: 79, b: 63};
pub const DESATURATED_ORANGE: Color = Color{r: 127, g: 95, b: 63};
pub const DESATURATED_AMBER: Color = Color{r: 127, g: 111, b: 63};
pub const DESATURATED_YELLOW: Color = Color{r: 127, g: 127, b: 63};
pub const DESATURATED_LIME: Color = Color{r: 111, g: 127, b: 63};
pub const DESATURATED_CHARTREUSE: Color = Color{r: 95, g: 127, b: 63};
pub const DESATURATED_GREEN: Color = Color{r: 63, g: 127, b: 63};
pub const DESATURATED_SEA: Color = Color{r: 63, g: 127, b: 95};
pub const DESATURATED_TURQUOISE: Color = Color{r: 63, g: 127, b: 111};
pub const DESATURATED_CYAN: Color = Color{r: 63, g: 127, b: 127};
pub const DESATURATED_SKY: Color = Color{r: 63, g: 111, b: 127};
pub const DESATURATED_AZURE: Color = Color{r: 63, g: 95, b: 127};
pub const DESATURATED_BLUE: Color = Color{r: 63, g: 63, b: 127};
pub const DESATURATED_HAN: Color = Color{r: 79, g: 63, b: 127};
pub const DESATURATED_VIOLET: Color = Color{r: 95, g: 63, b: 127};
pub const DESATURATED_PURPLE: Color = Color{r: 111, g: 63, b: 127};
pub const DESATURATED_FUCHSIA: Color = Color{r: 127, g: 63, b: 127};
pub const DESATURATED_MAGENTA: Color = Color{r: 127, g: 63, b: 111};
pub const DESATURATED_PINK: Color = Color{r: 127, g: 63, b: 95};
pub const DESATURATED_CRIMSON: Color = Color{r: 127, g: 63, b: 79};
pub const LIGHTEST_RED: Color = Color{r: 255, g: 191, b: 191};
pub const LIGHTEST_FLAME: Color = Color{r: 255, g: 207, b: 191};
pub const LIGHTEST_ORANGE: Color = Color{r: 255, g: 223, b: 191};
pub const LIGHTEST_AMBER: Color = Color{r: 255, g: 239, b: 191};
pub const LIGHTEST_YELLOW: Color = Color{r: 255, g: 255, b: 191};
pub const LIGHTEST_LIME: Color = Color{r: 239, g: 255, b: 191};
pub const LIGHTEST_CHARTREUSE: Color = Color{r: 223, g: 255, b: 191};
pub const LIGHTEST_GREEN: Color = Color{r: 191, g: 255, b: 191};
pub const LIGHTEST_SEA: Color = Color{r: 191, g: 255, b: 223};
pub const LIGHTEST_TURQUOISE: Color = Color{r: 191, g: 255, b: 239};
pub const LIGHTEST_CYAN: Color = Color{r: 191, g: 255, b: 255};
pub const LIGHTEST_SKY: Color = Color{r: 191, g: 239, b: 255};
pub const LIGHTEST_AZURE: Color = Color{r: 191, g: 223, b: 255};
pub const LIGHTEST_BLUE: Color = Color{r: 191, g: 191, b: 255};
pub const LIGHTEST_HAN: Color = Color{r: 207, g: 191, b: 255};
pub const LIGHTEST_VIOLET: Color = Color{r: 223, g: 191, b: 255};
pub const LIGHTEST_PURPLE: Color = Color{r: 239, g: 191, b: 255};
pub const LIGHTEST_FUCHSIA: Color = Color{r: 255, g: 191, b: 255};
pub const LIGHTEST_MAGENTA: Color = Color{r: 255, g: 191, b: 239};
pub const LIGHTEST_PINK: Color = Color{r: 255, g: 191, b: 223};
pub const LIGHTEST_CRIMSON: Color = Color{r: 255, g: 191, b: 207};
pub const LIGHTER_RED: Color = Color{r: 255, g: 127, b: 127};
pub const LIGHTER_FLAME: Color = Color{r: 255, g: 159, b: 127};
pub const LIGHTER_ORANGE: Color = Color{r: 255, g: 191, b: 127};
pub const LIGHTER_AMBER: Color = Color{r: 255, g: 223, b: 127};
pub const LIGHTER_YELLOW: Color = Color{r: 255, g: 255, b: 127};
pub const LIGHTER_LIME: Color = Color{r: 223, g: 255, b: 127};
pub const LIGHTER_CHARTREUSE: Color = Color{r: 191, g: 255, b: 127};
pub const LIGHTER_GREEN: Color = Color{r: 127, g: 255, b: 127};
pub const LIGHTER_SEA: Color = Color{r: 127, g: 255, b: 191};
pub const LIGHTER_TURQUOISE: Color = Color{r: 127, g: 255, b: 223};
pub const LIGHTER_CYAN: Color = Color{r: 127, g: 255, b: 255};
pub const LIGHTER_SKY: Color = Color{r: 127, g: 223, b: 255};
pub const LIGHTER_AZURE: Color = Color{r: 127, g: 191, b: 255};
pub const LIGHTER_BLUE: Color = Color{r: 127, g: 127, b: 255};
pub const LIGHTER_HAN: Color = Color{r: 159, g: 127, b: 255};
pub const LIGHTER_VIOLET: Color = Color{r: 191, g: 127, b: 255};
pub const LIGHTER_PURPLE: Color = Color{r: 223, g: 127, b: 255};
pub const LIGHTER_FUCHSIA: Color = Color{r: 255, g: 127, b: 255};
pub const LIGHTER_MAGENTA: Color = Color{r: 255, g: 127, b: 223};
pub const LIGHTER_PINK: Color = Color{r: 255, g: 127, b: 191};
pub const LIGHTER_CRIMSON: Color = Color{r: 255, g: 127, b: 159};
pub const LIGHT_RED: Color = Color{r: 255, g: 63, b: 63};
pub const LIGHT_FLAME: Color = Color{r: 255, g: 111, b: 63};
pub const LIGHT_ORANGE: Color = Color{r: 255, g: 159, b: 63};
pub const LIGHT_AMBER: Color = Color{r: 255, g: 207, b: 63};
pub const LIGHT_YELLOW: Color = Color{r: 255, g: 255, b: 63};
pub const LIGHT_LIME: Color = Color{r: 207, g: 255, b: 63};
pub const LIGHT_CHARTREUSE: Color = Color{r: 159, g: 255, b: 63};
pub const LIGHT_GREEN: Color = Color{r: 63, g: 255, b: 63};
pub const LIGHT_SEA: Color = Color{r: 63, g: 255, b: 159};
pub const LIGHT_TURQUOISE: Color = Color{r: 63, g: 255, b: 207};
pub const LIGHT_CYAN: Color = Color{r: 63, g: 255, b: 255};
pub const LIGHT_SKY: Color = Color{r: 63, g: 207, b: 255};
pub const LIGHT_AZURE: Color = Color{r: 63, g: 159, b: 255};
pub const LIGHT_BLUE: Color = Color{r: 63, g: 63, b: 255};
pub const LIGHT_HAN: Color = Color{r: 111, g: 63, b: 255};
pub const LIGHT_VIOLET: Color = Color{r: 159, g: 63, b: 255};
pub const LIGHT_PURPLE: Color = Color{r: 207, g: 63, b: 255};
pub const LIGHT_FUCHSIA: Color = Color{r: 255, g: 63, b: 255};
pub const LIGHT_MAGENTA: Color = Color{r: 255, g: 63, b: 207};
pub const LIGHT_PINK: Color = Color{r: 255, g: 63, b: 159};
pub const LIGHT_CRIMSON: Color = Color{r: 255, g: 63, b: 111};
pub const RED: Color = Color{r: 255, g: 0, b: 0};
pub const FLAME: Color = Color{r: 255, g: 63, b: 0};
pub const ORANGE: Color = Color{r: 255, g: 127, b: 0};
pub const AMBER: Color = Color{r: 255, g: 191, b: 0};
pub const YELLOW: Color = Color{r: 255, g: 255, b: 0};
pub const LIME: Color = Color{r: 191, g: 255, b: 0};
pub const CHARTREUSE: Color = Color{r: 127, g: 255, b: 0};
pub const GREEN: Color = Color{r: 0, g: 255, b: 0};
pub const SEA: Color = Color{r: 0, g: 255, b: 127};
pub const TURQUOISE: Color = Color{r: 0, g: 255, b: 191};
pub const CYAN: Color = Color{r: 0, g: 255, b: 255};
pub const SKY: Color = Color{r: 0, g: 191, b: 255};
pub const AZURE: Color = Color{r: 0, g: 127, b: 255};
pub const BLUE: Color = Color{r: 0, g: 0, b: 255};
pub const HAN: Color = Color{r: 63, g: 0, b: 255};
pub const VIOLET: Color = Color{r: 127, g: 0, b: 255};
pub const PURPLE: Color = Color{r: 191, g: 0, b: 255};
pub const FUCHSIA: Color = Color{r: 255, g: 0, b: 255};
pub const MAGENTA: Color = Color{r: 255, g: 0, b: 191};
pub const PINK: Color = Color{r: 255, g: 0, b: 127};
pub const CRIMSON: Color = Color{r: 255, g: 0, b: 63};
pub const DARK_RED: Color = Color{r: 191, g: 0, b: 0};
pub const DARK_FLAME: Color = Color{r: 191, g: 47, b: 0};
pub const DARK_ORANGE: Color = Color{r: 191, g: 95, b: 0};
pub const DARK_AMBER: Color = Color{r: 191, g: 143, b: 0};
pub const DARK_YELLOW: Color = Color{r: 191, g: 191, b: 0};
pub const DARK_LIME: Color = Color{r: 143, g: 191, b: 0};
pub const DARK_CHARTREUSE: Color = Color{r: 95, g: 191, b: 0};
pub const DARK_GREEN: Color = Color{r: 0, g: 191, b: 0};
pub const DARK_SEA: Color = Color{r: 0, g: 191, b: 95};
pub const DARK_TURQUOISE: Color = Color{r: 0, g: 191, b: 143};
pub const DARK_CYAN: Color = Color{r: 0, g: 191, b: 191};
pub const DARK_SKY: Color = Color{r: 0, g: 143, b: 191};
pub const DARK_AZURE: Color = Color{r: 0, g: 95, b: 191};
pub const DARK_BLUE: Color = Color{r: 0, g: 0, b: 191};
pub const DARK_HAN: Color = Color{r: 47, g: 0, b: 191};
pub const DARK_VIOLET: Color = Color{r: 95, g: 0, b: 191};
pub const DARK_PURPLE: Color = Color{r: 143, g: 0, b: 191};
pub const DARK_FUCHSIA: Color = Color{r: 191, g: 0, b: 191};
pub const DARK_MAGENTA: Color = Color{r: 191, g: 0, b: 143};
pub const DARK_PINK: Color = Color{r: 191, g: 0, b: 95};
pub const DARK_CRIMSON: Color = Color{r: 191, g: 0, b: 47};
pub const DARKER_RED: Color = Color{r: 127, g: 0, b: 0};
pub const DARKER_FLAME: Color = Color{r: 127, g: 31, b: 0};
pub const DARKER_ORANGE: Color = Color{r: 127, g: 63, b: 0};
pub const DARKER_AMBER: Color = Color{r: 127, g: 95, b: 0};
pub const DARKER_YELLOW: Color = Color{r: 127, g: 127, b: 0};
pub const DARKER_LIME: Color = Color{r: 95, g: 127, b: 0};
pub const DARKER_CHARTREUSE: Color = Color{r: 63, g: 127, b: 0};
pub const DARKER_GREEN: Color = Color{r: 0, g: 127, b: 0};
pub const DARKER_SEA: Color = Color{r: 0, g: 127, b: 63};
pub const DARKER_TURQUOISE: Color = Color{r: 0, g: 127, b: 95};
pub const DARKER_CYAN: Color = Color{r: 0, g: 127, b: 127};
pub const DARKER_SKY: Color = Color{r: 0, g: 95, b: 127};
pub const DARKER_AZURE: Color = Color{r: 0, g: 63, b: 127};
pub const DARKER_BLUE: Color = Color{r: 0, g: 0, b: 127};
pub const DARKER_HAN: Color = Color{r: 31, g: 0, b: 127};
pub const DARKER_VIOLET: Color = Color{r: 63, g: 0, b: 127};
pub const DARKER_PURPLE: Color = Color{r: 95, g: 0, b: 127};
pub const DARKER_FUCHSIA: Color = Color{r: 127, g: 0, b: 127};
pub const DARKER_MAGENTA: Color = Color{r: 127, g: 0, b: 95};
pub const DARKER_PINK: Color = Color{r: 127, g: 0, b: 63};
pub const DARKER_CRIMSON: Color = Color{r: 127, g: 0, b: 31};
pub const DARKEST_RED: Color = Color{r: 63, g: 0, b: 0};
pub const DARKEST_FLAME: Color = Color{r: 63, g: 15, b: 0};
pub const DARKEST_ORANGE: Color = Color{r: 63, g: 31, b: 0};
pub const DARKEST_AMBER: Color = Color{r: 63, g: 47, b: 0};
pub const DARKEST_YELLOW: Color = Color{r: 63, g: 63, b: 0};
pub const DARKEST_LIME: Color = Color{r: 47, g: 63, b: 0};
pub const DARKEST_CHARTREUSE: Color = Color{r: 31, g: 63, b: 0};
pub const DARKEST_GREEN: Color = Color{r: 0, g: 63, b: 0};
pub const DARKEST_SEA: Color = Color{r: 0, g: 63, b: 31};
pub const DARKEST_TURQUOISE: Color = Color{r: 0, g: 63, b: 47};
pub const DARKEST_CYAN: Color = Color{r: 0, g: 63, b: 63};
pub const DARKEST_SKY: Color = Color{r: 0, g: 47, b: 63};
pub const DARKEST_AZURE: Color = Color{r: 0, g: 31, b: 63};
pub const DARKEST_BLUE: Color = Color{r: 0, g: 0, b: 63};
pub const DARKEST_HAN: Color = Color{r: 15, g: 0, b: 63};
pub const DARKEST_VIOLET: Color = Color{r: 31, g: 0, b: 63};
pub const DARKEST_PURPLE: Color = Color{r: 47, g: 0, b: 63};
pub const DARKEST_FUCHSIA: Color = Color{r: 63, g: 0, b: 63};
pub const DARKEST_MAGENTA: Color = Color{r: 63, g: 0, b: 47};
pub const DARKEST_PINK: Color = Color{r: 63, g: 0, b: 31};
pub const DARKEST_CRIMSON: Color = Color{r: 63, g: 0, b: 15};
pub const BRASS: Color = Color{r: 191, g: 151, b: 96};
pub const COPPER: Color = Color{r: 197, g: 136, b: 124};
pub const GOLD: Color = Color{r: 229, g: 191, b: 0};
pub const SILVER: Color = Color{r: 203, g: 203, b: 203};
pub const CELADON: Color = Color{r: 172, g: 255, b: 175};
pub const PEACH: Color = Color{r: 255, g: 159, b: 127};
