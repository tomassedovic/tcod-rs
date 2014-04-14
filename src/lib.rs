#![crate_id = "tcod#0.1.0"]
#![crate_type = "lib"]
#![feature(globs)]

extern crate libc;

use std::cast;
use std::intrinsics::transmute;
use std::path;

use libc::{c_char, c_int, c_float, uint8_t};

pub mod ffi;

#[allow(non_camel_case_types)]
type c_bool = uint8_t;

pub struct Console {
    con: ffi::TCOD_console_t,
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

    pub fn init_root(width: int, height: int, title: &str, fullscreen: bool) -> Console {
        assert!(width > 0 && height > 0);
        unsafe {
            title.with_c_str(
                |c_title| ffi::TCOD_console_init_root(width as c_int, height as c_int,
                                                      c_title, fullscreen as c_bool,
                                                      ffi::TCOD_RENDERER_SDL));
        }
        Console{con: 0 as ffi::TCOD_console_t}
    }

    pub fn set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con, transmute(color));
        }
    }

    pub fn width(&self) -> int {
        unsafe {
            ffi::TCOD_console_get_width(self.con) as int
        }
    }

    pub fn height(&self) -> int {
        unsafe {
            ffi::TCOD_console_get_height(self.con) as int
        }
    }

    pub fn set_default_background(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_background(self.con, transmute(color));
        }
    }

    pub fn set_default_foreground(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_foreground(self.con, transmute(color));
        }
    }

    pub fn console_set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con, transmute(color));
        }
    }

    pub fn set_char_background(&mut self, x: int, y: int,
                               color: Color,
                               background_flag: background_flag::BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_background(self.con,
                                                  x as c_int, y as c_int,
                                                  transmute(color),
                                                  background_flag as u32)
        }
    }

    pub fn put_char(&mut self,
                    x: int, y: int, glyph: char,
                    background_flag: background_flag::BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char(self.con,
                                       x as c_int, y as c_int, glyph as c_int,
                                       background_flag as u32);
        }
    }

    pub fn put_char_ex(&mut self,
                       x: int, y: int, glyph: char,
                       foreground: Color, background: Color) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char_ex(self.con,
                                          x as c_int, y as c_int, glyph as c_int,
                                          transmute(foreground),
                                          transmute(background));
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::TCOD_console_clear(self.con);
        }
    }

    pub fn print_ex(&mut self,
                    x: int, y: int,
                    background_flag: background_flag::BackgroundFlag,
                    alignment: TextAlignment,
                    text: &str) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            text.with_c_str(
                |c_text|
                ffi::TCOD_console_print_ex(self.con,
                                           x as c_int, y as c_int,
                                           background_flag as u32,
                                           alignment as u32,
                                           c_text));
        }
    }

    pub fn set_fade(&mut self, fade: u8, fading_color: Color) {
        assert!(self.con == 0 as ffi::TCOD_console_t,
                "Fade can be set on the root console only.");
        unsafe {
            ffi::TCOD_console_set_fade(fade, transmute(fading_color));
        }
    }

    pub fn set_custom_font(&mut self, font_path: path::Path) {
        assert!(self.con == 0 as ffi::TCOD_console_t,
                "Custom font can be set on the root console only.");
        unsafe {
            let flags = LayoutTcod as c_int | TypeGreyscale as c_int;
            font_path.with_c_str( |path| {
                ffi::TCOD_console_set_custom_font(path, flags, 32, 8);
            });
        }
    }

    pub fn wait_for_keypress(&self, flush: bool) -> Key {
        assert!(self.con == 0 as ffi::TCOD_console_t,
                "wait_for_keypress can be called on the root console only.");
        unsafe {
            transmute(ffi::TCOD_console_wait_for_keypress(flush as c_bool))
        }
    }

    pub fn check_for_keypress(&self, status: KeyStatus) -> Key {
        assert!(self.con == 0 as ffi::TCOD_console_t,
                "check_for_keypress can be called on the root console only.");
        unsafe {
            transmute(ffi::TCOD_console_check_for_keypress(status as c_int))
        }
    }

    pub fn window_closed(&mut self) -> bool {
        assert!(self.con == 0 as ffi::TCOD_console_t,
                "Window checks can be done on the root console only.");
        unsafe {
            ffi::TCOD_console_is_window_closed() != 0
        }
    }

    pub fn flush(&mut self) {
        assert!(self.con == 0 as ffi::TCOD_console_t,
                "Only the root console can be flushed.");
        unsafe {
            ffi::TCOD_console_flush();
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
    tcod_map: ffi::TCOD_map_t,
}

pub struct Path {
    tcod_path: ffi::TCOD_path_t,
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
pub enum Renderer {
    GLSL = ffi::TCOD_RENDERER_GLSL,
    OpenGL = ffi::TCOD_RENDERER_OPENGL,
    SDL = ffi::TCOD_RENDERER_SDL,
}

#[repr(C)]
pub enum FontFlags {
    LayoutAsciiIncol = ffi::TCOD_FONT_LAYOUT_ASCII_INCOL,
    LayoutAsciiInrow = ffi::TCOD_FONT_LAYOUT_ASCII_INROW,
    TypeGreyscale = ffi::TCOD_FONT_TYPE_GREYSCALE,
    LayoutTcod = ffi::TCOD_FONT_LAYOUT_TCOD,
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

pub mod background_flag {
    use super::ffi;

    #[repr(C)]
    pub enum BackgroundFlag {
        None = ffi::TCOD_BKGND_NONE,
        Set = ffi::TCOD_BKGND_SET,
        Multiply = ffi::TCOD_BKGND_MULTIPLY,
        Lighten = ffi::TCOD_BKGND_LIGHTEN,
        Darken = ffi::TCOD_BKGND_DARKEN,
        Screen = ffi::TCOD_BKGND_SCREEN,
        ColorDodge = ffi::TCOD_BKGND_COLOR_DODGE,
        ColorBurn = ffi::TCOD_BKGND_COLOR_BURN,
        Add = ffi::TCOD_BKGND_ADD,
        AddA = ffi::TCOD_BKGND_ADDA,
        Burn = ffi::TCOD_BKGND_BURN,
        Overlay = ffi::TCOD_BKGND_OVERLAY,
        Alph = ffi::TCOD_BKGND_ALPH,
        Default = ffi::TCOD_BKGND_DEFAULT
    }
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


pub enum KeyStatus {
    KeyPressed = 1,
    KeyReleased = 2,
    KeyPressedOrReleased = 1 | 2,
}

pub fn console_blit(source_console: &Console,
                    source_x: int, source_y: int,
                    source_width: int, source_height: int,
                    destination_console: &mut Console,
                    destination_x: int, destination_y: int,
                    foreground_alpha: f32, background_alpha: f32) {
    assert!(source_x >= 0 && source_y >= 0 &&
            source_width > 0 && source_height > 0 &&
            destination_x >= 0 && destination_y >= 0);
    unsafe {
        ffi::TCOD_console_blit(source_console.con, source_x as c_int, source_y as c_int,
                               source_width as c_int, source_height as c_int,
                               destination_console.con,
                               destination_x as c_int, destination_y as c_int,
                               foreground_alpha as c_float,
                               background_alpha as c_float);
    }
}
