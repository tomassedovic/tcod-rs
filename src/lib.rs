#[feature(globs)];

use std::cast;
use std::libc::{c_char, c_int, c_float, uint8_t};
use std::intrinsics::transmute;
use std::path;

//pub use self::ffi::Color;
//pub use self::ffi::console_t;
//pub use self::ffi::BKGND_NONE;
//pub use self::ffi::key_code;
//pub use self::ffi::Key;
//pub use self::ffi::*;
//pub use self::ffi::Right;

//use self::ffi::{c_int, c_float, c_bool};

pub mod ffi;

#[allow(non_camel_case_types)]
type c_bool = std::libc::uint8_t;

pub struct Console {
    priv con: ffi::TCOD_console_t,
}

impl Console {
    pub fn new(width: int, height: int) -> Console {
        assert!(width > 0 && height > 0);
        unsafe {
            Console{
                con: ffi::TCOD_console_new(width as c_int, height as c_int)
            }
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_console_delete(self.con);
        }
    }
}

pub struct Map {
    priv tcod_map: ffi::TCOD_map_t,
}

pub struct Path {
    priv tcod_path: ffi::TCOD_path_t,
}

impl Map {
    pub fn new(width: int, height: int) -> Map {
        assert!(width > 0 && height > 0);
        unsafe {
            Map{tcod_map: ffi::TCOD_map_new(width as c_int, height as c_int)}
        }
    }

    pub fn size(&self) -> (int, int) {
        unsafe {
            (ffi::TCOD_map_get_width(self.tcod_map) as int,
             ffi::TCOD_map_get_height(self.tcod_map) as int)
        }
    }

    pub fn set(&mut self, x: int, y: int, transparent: bool, walkable: bool) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_set_properties(self.tcod_map, x as c_int, y as c_int,
                                         transparent as c_bool,
                                         walkable as c_bool);
        }
    }

    pub fn is_walkable(&self, x: int, y: int) -> bool {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_is_walkable(self.tcod_map, x as c_int, y as c_int) != 0
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_map_delete(self.tcod_map)
        }
    }
}


impl Path {
    pub fn new_using_map(map: Map, diagonal_cost: f32) -> Path {
        unsafe {
            Path {
                tcod_path: ffi::TCOD_path_new_using_map(map.tcod_map,
                                                        diagonal_cost as c_float)
            }
        }
    }

    pub fn new_using_function<T>(map_width: int, map_height: int,
                                 path_cb: ffi::TCOD_path_func_t,
                                 user_data: &T,
                                 diagonal_cost: f32) -> Path {
        assert!(map_width > 0 && map_height > 0);
        unsafe {
            Path {
                tcod_path: ffi::TCOD_path_new_using_function(map_width as c_int,
                                                             map_height as c_int,
                                                             path_cb,
                                                             cast::transmute(user_data),
                                                             diagonal_cost as c_float)
            }
        }
    }

    pub fn find(&mut self,
                from_x: int, from_y: int,
                to_x: int, to_y: int)
                -> bool {
        assert!(from_x >= 0 && from_y >= 0 && to_x >= 0 && to_y >= 0);
        unsafe {
            ffi::TCOD_path_compute(self.tcod_path,
                                   from_x as c_int, from_y as c_int,
                                   to_x as c_int, to_y as c_int) != 0
        }
    }

    pub fn walk(&mut self, recalculate_when_needed: bool)
                -> Option<(int, int)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path, &mut x, &mut y,
                                      recalculate_when_needed as c_bool) != 0 {
                true => Some((x as int, y as int)),
                false => None,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_path_is_empty(self.tcod_path) != 0
        }
    }

    pub fn len(&self) -> int {
        unsafe {
            ffi::TCOD_path_size(self.tcod_path) as int
        }
    }

    pub fn destination(&self) -> (int, int) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_destination(self.tcod_path, &mut x, &mut y);
            (x as int, y as int)
        }
    }

}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_path_delete(self.tcod_path);
        }
    }
}


// TODO: exlpcitly add the correct FFI values to the enums


#[repr(C)]
pub enum renderer_t {
    RENDERER_GLSL,
    RENDERER_OPENGL,
    RENDERER_SDL,
    NB_RENDERERS,
}

#[repr(C)]
pub enum font_flags_t {
    FONT_LAYOUT_ASCII_INCOL=1,
    FONT_LAYOUT_ASCII_INROW=2,
    FONT_TYPE_GREYSCALE=4,
    FONT_LAYOUT_TCOD=8,
}

pub mod key_code {
    #[deriving(Eq, Show)]
    #[repr(C)]
    pub enum KeyCode {
        NoKey,
        Escape,
        Backspace,
        Tab,
        Enter,
        Shift,
        Control,
        Alt,
        Pause,
        Capslock,
        Pageup,
        Pagedown,
        End,
        Home,
        Up,
        Left,
        Right,
        Down,
        PrintScreen,
        Insert,
        Delete,
        LeftWin,
        RightWin,
        Apps,
        // The numbers on the alphanum section of the keyboard
        Number0,
        Number1,
        Number2,
        Number3,
        Number4,
        Number5,
        Number6,
        Number7,
        Number8,
        Number9,
        // The numbers on the numeric keypad
        NumPad0,
        NumPad1,
        NumPad2,
        NumPad3,
        NumPad4,
        NumPad5,
        NumPad6,
        NumPad7,
        NumPad8,
        NumPad9,
        NumPadAdd,
        NumPadSubtract,
        NumPadDivide,
        NumPadMultiply,
        NumPadDecimal,
        NumPadEnter,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        NUMLOCK,
        SCROLLLOCK,
        Spacebar,
        Char,
    }
}

#[repr(C)]
pub struct Key {
    vk: key_code::KeyCode,
    c: c_char,
    pressed: c_bool,
    lalt: c_bool,
    lctrl: c_bool,
    ralt: c_bool,
    rctrl: c_bool,
    shift: c_bool,
}

#[deriving(Clone, Eq)]
#[repr(C)]
pub struct Color {
    r: uint8_t,
    g: uint8_t,
    b: uint8_t,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color{r: red as uint8_t, g: green as uint8_t, b: blue as uint8_t}
    }
}

#[repr(C)]
pub enum TextAlignment {
    Left,
    Right,
    Center,
}

#[repr(C)]
pub enum BackgroundFlag {
    BKGND_NONE,
    BKGND_SET,
    BKGND_MULTIPLY,
    BKGND_LIGHTEN,
    BKGND_DARKEN,
    BKGND_SCREEN,
    BKGND_COLOR_DODGE,
    BKGND_COLOR_BURN,
    BKGND_ADD,
    BKGND_ADDA,
    BKGND_BURN,
    BKGND_OVERLAY,
    BKGND_ALPH,
    BKGND_DEFAULT
}


pub fn sys_set_fps(fps: int) {
    assert!(fps > 0);
    unsafe {
        ffi::TCOD_sys_set_fps(fps as c_int)
    }
}

pub fn sys_get_fps() -> int {
    let mut result;
    unsafe {
        result = ffi::TCOD_sys_get_fps();
    }
    assert!(result >= 0);
    return result as int
}

pub fn sys_get_last_frame_length() -> f32 {
    unsafe {
        ffi::TCOD_sys_get_last_frame_length() as f32
    }
}

pub fn console_init_root(width: int, height: int, title: &str, fullscreen: bool) {
    assert!(width > 0 && height > 0);
    unsafe {
        title.with_c_str(
            |c_title| ffi::TCOD_console_init_root(width as c_int, height as c_int,
                                                  c_title, fullscreen as c_bool,
                                                  ffi::TCOD_RENDERER_SDL));
    }
}

pub fn console_set_fade(fade: u8, fading_color: Color) {
    unsafe {
        ffi::TCOD_console_set_fade(fade, transmute(fading_color));
    }
}

pub fn console_set_custom_font(font_path: path::Path) {
    unsafe {
        let flags = (ffi::TCOD_FONT_TYPE_GREYSCALE as c_int |
                     ffi::TCOD_FONT_LAYOUT_TCOD as c_int);
        font_path.with_c_str( |path| {
            ffi::TCOD_console_set_custom_font(path, flags, 32, 8);
        });
    }
}

pub fn console_is_window_closed() -> bool {
    unsafe {
        ffi::TCOD_console_is_window_closed() != 0
    }
}

pub enum KeyStatus {
    KeyPressed = 1,
    KeyReleased = 2,
    KeyPressedOrReleased = 1 | 2,
}

pub fn console_wait_for_keypress(flush: bool) -> Key {
    unsafe {
        transmute(ffi::TCOD_console_wait_for_keypress(flush as c_bool))
    }
}

pub fn console_check_for_keypress(status: KeyStatus) -> Key {
    unsafe {
        transmute(ffi::TCOD_console_check_for_keypress(status as c_int))
    }
}

/*
pub fn console_set_char_background(con: console_t, x: int, y: int,
                                   color: Color,
                                   background_flag: BackgroundFlag) {
    assert!(x >= 0 && y >= 0);
    unsafe {
        ffi::TCOD_console_set_char_background(con, x as c_int, y as c_int,
                                              color, background_flag)
    }
}

pub fn console_put_char(con: console_t, x: int, y: int, glyph: char,
                        background_flag: ffi::BackgroundFlag) {
    assert!(x >= 0 && y >= 0);
    unsafe {
        ffi::TCOD_console_put_char(con, x as c_int, y as c_int, glyph as c_int,
                                   background_flag);
    }
}

pub fn console_put_char_ex(con: console_t, x: int, y: int, glyph: char,
                           foreground: Color, background: Color) {
    assert!(x >= 0 && y >= 0);
    unsafe {
        ffi::TCOD_console_put_char_ex(con, x as c_int, y as c_int, glyph as c_int,
                                      foreground, background);
    }
}

pub fn console_clear(con: console_t) {
    unsafe {
        ffi::TCOD_console_clear(con);
    }
}

pub fn console_flush() {
    unsafe {
        ffi::TCOD_console_flush();
    }
}

pub fn console_print_ex(con: console_t, x: int, y: int,
                        background_flag: ffi::BackgroundFlag,
                        alignment: ffi::TextAlignment,
                        text: &str) {
    assert!(x >= 0 && y >= 0);
    unsafe {
        text.with_c_str(
            |c_text|
                ffi::TCOD_console_print_ex(con, x as c_int, y as c_int,
                                           background_flag, alignment, c_text));
    }
}

pub fn console_get_width(con: console_t) -> int {
    unsafe {
        ffi::TCOD_console_get_width(con) as int
    }
}

pub fn console_get_height(con: console_t) -> int {
    unsafe {
        ffi::TCOD_console_get_height(con) as int
    }
}

pub fn console_set_default_background(con: console_t, color: Color) {
    unsafe {
        ffi::TCOD_console_set_default_background(con, color);
    }
}

pub fn console_set_default_foreground(con: console_t, color: Color) {
    unsafe {
        ffi::TCOD_console_set_default_foreground(con, color);
    }
}

pub fn console_set_key_color(con: console_t, color: Color) {
    unsafe {
        ffi::TCOD_console_set_key_color(con, color);
    }
}

pub fn console_blit(source_console: console_t,
                    source_x: int, source_y: int,
                    source_width: int, source_height: int,
                    destination_console: console_t,
                    destination_x: int, destination_y: int,
                    foreground_alpha: f32, background_alpha: f32) {
    assert!(source_x >= 0 && source_y >= 0 &&
            source_width > 0 && source_height > 0 &&
            destination_x >= 0 && destination_y >= 0);
    unsafe {
        ffi::TCOD_console_blit(source_console, source_x as c_int, source_y as c_int,
                               source_width as c_int, source_height as c_int,
                               destination_console,
                               destination_x as c_int, destination_y as c_int,
                               foreground_alpha as c_float,
                               background_alpha as c_float);
    }
}
*/
