use std::ops::{Add, Sub, Div, Mul};
use bindings::{AsNative, FromNative, ffi};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl AsNative<ffi::TCOD_color_t> for Color {
    unsafe fn as_native(&self) -> &ffi::TCOD_color_t {
        ::std::mem::transmute(self)
    }
    
    unsafe fn as_native_mut(&mut self) -> &mut ffi::TCOD_color_t {
        ::std::mem::transmute(self)
    }
}

impl FromNative<ffi::TCOD_color_t> for Color {
    unsafe fn from_native(input: ffi::TCOD_color_t) -> Color {
        ::std::mem::transmute(input)
    }
}

impl Color {
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

    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn new_from_hsv(h: f32, s: f32, v: f32) -> Color {
        unsafe {
            FromNative::from_native(ffi::TCOD_color_HSV(h, s, v))
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

pub fn lerp(from: Color, to: Color, coefficient: f32) -> Color {
    unsafe {
        FromNative::from_native(ffi::TCOD_color_lerp(*from.as_native(),
                                                     *to.as_native(),
                                                     coefficient))
    }
}

impl Add<Color> for Color {
    type Output = Color;

    #[inline]
    fn add(self, rhs: Color) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_color_add(*self.as_native(), *rhs.as_native()))
        }
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    #[inline]
    fn sub(self, rhs: Color) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_color_subtract(*self.as_native(), *rhs.as_native()))
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_color_multiply(*self.as_native(), *rhs.as_native()))
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: f32) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_color_multiply_scalar(*self.as_native(), rhs))
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    #[inline]
    fn mul(self, rhs: Color) -> Color {
        rhs * self
    }
}

impl Div<f32> for Color {
    type Output = Color;

    #[inline]
    fn div(self, rhs: f32) -> Color {
        self * (1.0 / rhs)
    }
}

// NOTE; colour names and values copied from:
// tcod-sys/libtcod/include/libtcod_int.h
//
// We cannot return statics exported by the DLL here because they have a
// different type (TCOD_color_t) and we cannot call `transmute` to convert
// them to `Color`.
pub const BLACK: Color = Color::BLACK;
pub const DARKEST_GREY: Color = Color::DARKEST_GREY;
pub const DARKER_GREY: Color = Color::DARKER_GREY;
pub const DARK_GREY: Color = Color::DARK_GREY;
pub const GREY: Color = Color::GREY;
pub const LIGHT_GREY: Color = Color::LIGHT_GREY;
pub const LIGHTER_GREY: Color = Color::LIGHTER_GREY;
pub const LIGHTEST_GREY: Color = Color::LIGHTEST_GREY;
pub const WHITE: Color = Color::WHITE;
pub const DARKEST_SEPIA: Color = Color::DARKEST_SEPIA;
pub const DARKER_SEPIA: Color = Color::DARKER_SEPIA;
pub const DARK_SEPIA: Color = Color::DARK_SEPIA;
pub const SEPIA: Color = Color::SEPIA;
pub const LIGHT_SEPIA: Color = Color::LIGHT_SEPIA;
pub const LIGHTER_SEPIA: Color = Color::LIGHTER_SEPIA;
pub const LIGHTEST_SEPIA: Color = Color::LIGHTEST_SEPIA;
pub const DESATURATED_RED: Color = Color::DESATURATED_RED;
pub const DESATURATED_FLAME: Color = Color::DESATURATED_FLAME;
pub const DESATURATED_ORANGE: Color = Color::DESATURATED_ORANGE;
pub const DESATURATED_AMBER: Color = Color::DESATURATED_AMBER;
pub const DESATURATED_YELLOW: Color = Color::DESATURATED_YELLOW;
pub const DESATURATED_LIME: Color = Color::DESATURATED_LIME;
pub const DESATURATED_CHARTREUSE: Color = Color::DESATURATED_CHARTREUSE;
pub const DESATURATED_GREEN: Color = Color::DESATURATED_GREEN;
pub const DESATURATED_SEA: Color = Color::DESATURATED_SEA;
pub const DESATURATED_TURQUOISE: Color = Color::DESATURATED_TURQUOISE;
pub const DESATURATED_CYAN: Color = Color::DESATURATED_CYAN;
pub const DESATURATED_SKY: Color = Color::DESATURATED_SKY;
pub const DESATURATED_AZURE: Color = Color::DESATURATED_AZURE;
pub const DESATURATED_BLUE: Color = Color::DESATURATED_BLUE;
pub const DESATURATED_HAN: Color = Color::DESATURATED_HAN;
pub const DESATURATED_VIOLET: Color = Color::DESATURATED_VIOLET;
pub const DESATURATED_PURPLE: Color = Color::DESATURATED_PURPLE;
pub const DESATURATED_FUCHSIA: Color = Color::DESATURATED_FUCHSIA;
pub const DESATURATED_MAGENTA: Color = Color::DESATURATED_MAGENTA;
pub const DESATURATED_PINK: Color = Color::DESATURATED_PINK;
pub const DESATURATED_CRIMSON: Color = Color::DESATURATED_CRIMSON;
pub const LIGHTEST_RED: Color = Color::LIGHTEST_RED;
pub const LIGHTEST_FLAME: Color = Color::LIGHTEST_FLAME;
pub const LIGHTEST_ORANGE: Color = Color::LIGHTEST_ORANGE;
pub const LIGHTEST_AMBER: Color = Color::LIGHTEST_AMBER;
pub const LIGHTEST_YELLOW: Color = Color::LIGHTEST_YELLOW;
pub const LIGHTEST_LIME: Color = Color::LIGHTEST_LIME;
pub const LIGHTEST_CHARTREUSE: Color = Color::LIGHTEST_CHARTREUSE;
pub const LIGHTEST_GREEN: Color = Color::LIGHTEST_GREEN;
pub const LIGHTEST_SEA: Color = Color::LIGHTEST_SEA;
pub const LIGHTEST_TURQUOISE: Color = Color::LIGHTEST_TURQUOISE;
pub const LIGHTEST_CYAN: Color = Color::LIGHTEST_CYAN;
pub const LIGHTEST_SKY: Color = Color::LIGHTEST_SKY;
pub const LIGHTEST_AZURE: Color = Color::LIGHTEST_AZURE;
pub const LIGHTEST_BLUE: Color = Color::LIGHTEST_BLUE;
pub const LIGHTEST_HAN: Color = Color::LIGHTEST_HAN;
pub const LIGHTEST_VIOLET: Color = Color::LIGHTEST_VIOLET;
pub const LIGHTEST_PURPLE: Color = Color::LIGHTEST_PURPLE;
pub const LIGHTEST_FUCHSIA: Color = Color::LIGHTEST_FUCHSIA;
pub const LIGHTEST_MAGENTA: Color = Color::LIGHTEST_MAGENTA;
pub const LIGHTEST_PINK: Color = Color::LIGHTEST_PINK;
pub const LIGHTEST_CRIMSON: Color = Color::LIGHTEST_CRIMSON;
pub const LIGHTER_RED: Color = Color::LIGHTER_RED;
pub const LIGHTER_FLAME: Color = Color::LIGHTER_FLAME;
pub const LIGHTER_ORANGE: Color = Color::LIGHTER_ORANGE;
pub const LIGHTER_AMBER: Color = Color::LIGHTER_AMBER;
pub const LIGHTER_YELLOW: Color = Color::LIGHTER_YELLOW;
pub const LIGHTER_LIME: Color = Color::LIGHTER_LIME;
pub const LIGHTER_CHARTREUSE: Color = Color::LIGHTER_CHARTREUSE;
pub const LIGHTER_GREEN: Color = Color::LIGHTER_GREEN;
pub const LIGHTER_SEA: Color = Color::LIGHTER_SEA;
pub const LIGHTER_TURQUOISE: Color = Color::LIGHTER_TURQUOISE;
pub const LIGHTER_CYAN: Color = Color::LIGHTER_CYAN;
pub const LIGHTER_SKY: Color = Color::LIGHTER_SKY;
pub const LIGHTER_AZURE: Color = Color::LIGHTER_AZURE;
pub const LIGHTER_BLUE: Color = Color::LIGHTER_BLUE;
pub const LIGHTER_HAN: Color = Color::LIGHTER_HAN;
pub const LIGHTER_VIOLET: Color = Color::LIGHTER_VIOLET;
pub const LIGHTER_PURPLE: Color = Color::LIGHTER_PURPLE;
pub const LIGHTER_FUCHSIA: Color = Color::LIGHTER_FUCHSIA;
pub const LIGHTER_MAGENTA: Color = Color::LIGHTER_MAGENTA;
pub const LIGHTER_PINK: Color = Color::LIGHTER_PINK;
pub const LIGHTER_CRIMSON: Color = Color::LIGHTER_CRIMSON;
pub const LIGHT_RED: Color = Color::LIGHT_RED;
pub const LIGHT_FLAME: Color = Color::LIGHT_FLAME;
pub const LIGHT_ORANGE: Color = Color::LIGHT_ORANGE;
pub const LIGHT_AMBER: Color = Color::LIGHT_AMBER;
pub const LIGHT_YELLOW: Color = Color::LIGHT_YELLOW;
pub const LIGHT_LIME: Color = Color::LIGHT_LIME;
pub const LIGHT_CHARTREUSE: Color = Color::LIGHT_CHARTREUSE;
pub const LIGHT_GREEN: Color = Color::LIGHT_GREEN;
pub const LIGHT_SEA: Color = Color::LIGHT_SEA;
pub const LIGHT_TURQUOISE: Color = Color::LIGHT_TURQUOISE;
pub const LIGHT_CYAN: Color = Color::LIGHT_CYAN;
pub const LIGHT_SKY: Color = Color::LIGHT_SKY;
pub const LIGHT_AZURE: Color = Color::LIGHT_AZURE;
pub const LIGHT_BLUE: Color = Color::LIGHT_BLUE;
pub const LIGHT_HAN: Color = Color::LIGHT_HAN;
pub const LIGHT_VIOLET: Color = Color::LIGHT_VIOLET;
pub const LIGHT_PURPLE: Color = Color::LIGHT_PURPLE;
pub const LIGHT_FUCHSIA: Color = Color::LIGHT_FUCHSIA;
pub const LIGHT_MAGENTA: Color = Color::LIGHT_MAGENTA;
pub const LIGHT_PINK: Color = Color::LIGHT_PINK;
pub const LIGHT_CRIMSON: Color = Color::LIGHT_CRIMSON;
pub const RED: Color = Color::RED;
pub const FLAME: Color = Color::FLAME;
pub const ORANGE: Color = Color::ORANGE;
pub const AMBER: Color = Color::AMBER;
pub const YELLOW: Color = Color::YELLOW;
pub const LIME: Color = Color::LIME;
pub const CHARTREUSE: Color = Color::CHARTREUSE;
pub const GREEN: Color = Color::GREEN;
pub const SEA: Color = Color::SEA;
pub const TURQUOISE: Color = Color::TURQUOISE;
pub const CYAN: Color = Color::CYAN;
pub const SKY: Color = Color::SKY;
pub const AZURE: Color = Color::AZURE;
pub const BLUE: Color = Color::BLUE;
pub const HAN: Color = Color::HAN;
pub const VIOLET: Color = Color::VIOLET;
pub const PURPLE: Color = Color::PURPLE;
pub const FUCHSIA: Color = Color::FUCHSIA;
pub const MAGENTA: Color = Color::MAGENTA;
pub const PINK: Color = Color::PINK;
pub const CRIMSON: Color = Color::CRIMSON;
pub const DARK_RED: Color = Color::DARK_RED;
pub const DARK_FLAME: Color = Color::DARK_FLAME;
pub const DARK_ORANGE: Color = Color::DARK_ORANGE;
pub const DARK_AMBER: Color = Color::DARK_AMBER;
pub const DARK_YELLOW: Color = Color::DARK_YELLOW;
pub const DARK_LIME: Color = Color::DARK_LIME;
pub const DARK_CHARTREUSE: Color = Color::DARK_CHARTREUSE;
pub const DARK_GREEN: Color = Color::DARK_GREEN;
pub const DARK_SEA: Color = Color::DARK_SEA;
pub const DARK_TURQUOISE: Color = Color::DARK_TURQUOISE;
pub const DARK_CYAN: Color = Color::DARK_CYAN;
pub const DARK_SKY: Color = Color::DARK_SKY;
pub const DARK_AZURE: Color = Color::DARK_AZURE;
pub const DARK_BLUE: Color = Color::DARK_BLUE;
pub const DARK_HAN: Color = Color::DARK_HAN;
pub const DARK_VIOLET: Color = Color::DARK_VIOLET;
pub const DARK_PURPLE: Color = Color::DARK_PURPLE;
pub const DARK_FUCHSIA: Color = Color::DARK_FUCHSIA;
pub const DARK_MAGENTA: Color = Color::DARK_MAGENTA;
pub const DARK_PINK: Color = Color::DARK_PINK;
pub const DARK_CRIMSON: Color = Color::DARK_CRIMSON;
pub const DARKER_RED: Color = Color::DARKER_RED;
pub const DARKER_FLAME: Color = Color::DARKER_FLAME;
pub const DARKER_ORANGE: Color = Color::DARKER_ORANGE;
pub const DARKER_AMBER: Color = Color::DARKER_AMBER;
pub const DARKER_YELLOW: Color = Color::DARKER_YELLOW;
pub const DARKER_LIME: Color = Color::DARKER_LIME;
pub const DARKER_CHARTREUSE: Color = Color::DARKER_CHARTREUSE;
pub const DARKER_GREEN: Color = Color::DARKER_GREEN;
pub const DARKER_SEA: Color = Color::DARKER_SEA;
pub const DARKER_TURQUOISE: Color = Color::DARKER_TURQUOISE;
pub const DARKER_CYAN: Color = Color::DARKER_CYAN;
pub const DARKER_SKY: Color = Color::DARKER_SKY;
pub const DARKER_AZURE: Color = Color::DARKER_AZURE;
pub const DARKER_BLUE: Color = Color::DARKER_BLUE;
pub const DARKER_HAN: Color = Color::DARKER_HAN;
pub const DARKER_VIOLET: Color = Color::DARKER_VIOLET;
pub const DARKER_PURPLE: Color = Color::DARKER_PURPLE;
pub const DARKER_FUCHSIA: Color = Color::DARKER_FUCHSIA;
pub const DARKER_MAGENTA: Color = Color::DARKER_MAGENTA;
pub const DARKER_PINK: Color = Color::DARKER_PINK;
pub const DARKER_CRIMSON: Color = Color::DARKER_CRIMSON;
pub const DARKEST_RED: Color = Color::DARKEST_RED;
pub const DARKEST_FLAME: Color = Color::DARKEST_FLAME;
pub const DARKEST_ORANGE: Color = Color::DARKEST_ORANGE;
pub const DARKEST_AMBER: Color = Color::DARKEST_AMBER;
pub const DARKEST_YELLOW: Color = Color::DARKEST_YELLOW;
pub const DARKEST_LIME: Color = Color::DARKEST_LIME;
pub const DARKEST_CHARTREUSE: Color = Color::DARKEST_CHARTREUSE;
pub const DARKEST_GREEN: Color = Color::DARKEST_GREEN;
pub const DARKEST_SEA: Color = Color::DARKEST_SEA;
pub const DARKEST_TURQUOISE: Color = Color::DARKEST_TURQUOISE;
pub const DARKEST_CYAN: Color = Color::DARKEST_CYAN;
pub const DARKEST_SKY: Color = Color::DARKEST_SKY;
pub const DARKEST_AZURE: Color = Color::DARKEST_AZURE;
pub const DARKEST_BLUE: Color = Color::DARKEST_BLUE;
pub const DARKEST_HAN: Color = Color::DARKEST_HAN;
pub const DARKEST_VIOLET: Color = Color::DARKEST_VIOLET;
pub const DARKEST_PURPLE: Color = Color::DARKEST_PURPLE;
pub const DARKEST_FUCHSIA: Color = Color::DARKEST_FUCHSIA;
pub const DARKEST_MAGENTA: Color = Color::DARKEST_MAGENTA;
pub const DARKEST_PINK: Color = Color::DARKEST_PINK;
pub const DARKEST_CRIMSON: Color = Color::DARKEST_CRIMSON;
pub const BRASS: Color = Color::BRASS;
pub const COPPER: Color = Color::COPPER;
pub const GOLD: Color = Color::GOLD;
pub const SILVER: Color = Color::SILVER;
pub const CELADON: Color = Color::CELADON;
pub const PEACH: Color = Color::PEACH;


#[cfg(all(feature = "serialization", test))]
mod test_serialization {
    use colors::Color;
    use serde_json;

    #[test]
    fn color_encode() {
        let encoded = serde_json::to_string(&Color{r: 1, g: 2, b: 3}).unwrap();
        assert_eq!("{\"r\":1,\"g\":2,\"b\":3}", encoded);
    }

    #[test]
    fn color_decode() {
        let decoded: Color = serde_json::from_str("{\"r\":1,\"g\":2,\"b\":3}").unwrap();
        assert_eq!(Color{r: 1, g: 2, b: 3}, decoded);
    }
}
