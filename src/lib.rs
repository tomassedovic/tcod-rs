#![feature(globs, unsafe_destructor)]

extern crate libc;

use std::mem::{transmute, transmute_copy};

use libc::{c_int, c_float, uint8_t, c_void};

#[allow(non_camel_case_types, non_snake_case)]
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


    pub fn blit(source_console: &Console,
                source_x: int, source_y: int,
                source_width: int, source_height: int,
                destination_console: &mut Console,
                destination_x: int, destination_y: int,
                foreground_alpha: f32, background_alpha: f32) {
        assert!(source_x >= 0 && source_y >= 0 &&
                source_width > 0 && source_height > 0 &&
                destination_x >= 0 && destination_y >= 0);
        unsafe {
            ffi::TCOD_console_blit(source_console.con,
                                   source_x as c_int, source_y as c_int,
                                   source_width as c_int, source_height as c_int,
                                   destination_console.con,
                                   destination_x as c_int, destination_y as c_int,
                                   foreground_alpha as c_float,
                                   background_alpha as c_float)
        }
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

    pub fn set_custom_font(font_path: ::std::path::Path) {
        unsafe {
            let flags = LayoutTcod as c_int | TypeGreyscale as c_int;
            font_path.with_c_str( |path| {
                ffi::TCOD_console_set_custom_font(path, flags, 32, 8);
            });
        }
    }

    pub fn wait_for_keypress(&self, flush: bool) -> KeyState {
        assert!(self.con == 0 as ffi::TCOD_console_t,
                "wait_for_keypress can be called on the root console only.");
        let tcod_key = unsafe {
            ffi::TCOD_console_wait_for_keypress(flush as c_bool)
        };
        assert!(tcod_key.vk != ffi::TCODK_NONE);
        let key = if tcod_key.vk == ffi::TCODK_CHAR {
            Printable(tcod_key.c as u8 as char)
        } else {
            Special(FromPrimitive::from_u32(tcod_key.vk).unwrap())
        };
        KeyState{
            key: key,
            pressed: tcod_key.pressed != 0,
            left_alt: tcod_key.lalt != 0,
            left_ctrl: tcod_key.lctrl != 0,
            right_alt: tcod_key.ralt != 0,
            right_ctrl: tcod_key.rctrl != 0,
            shift: tcod_key.shift != 0,
        }
    }

    pub fn check_for_keypress(&self, status: KeyPressFlag) -> Option<KeyState> {
        assert!(self.con == 0 as ffi::TCOD_console_t,
                "check_for_keypress can be called on the root console only.");
        let tcod_key = unsafe {
            ffi::TCOD_console_check_for_keypress(status as c_int)
        };
        if tcod_key.vk == ffi::TCODK_NONE {
            return None;
        }
        let key = if tcod_key.vk == ffi::TCODK_CHAR {
            Printable(tcod_key.c as u8 as char)
        } else {
            Special(FromPrimitive::from_u32(tcod_key.vk).unwrap())
        };
        Some(KeyState{
            key: key,
            pressed: tcod_key.pressed != 0,
            left_alt: tcod_key.lalt != 0,
            left_ctrl: tcod_key.lctrl != 0,
            right_alt: tcod_key.ralt != 0,
            right_ctrl: tcod_key.rctrl != 0,
            shift: tcod_key.shift != 0,
        })
    }

    pub fn window_closed() -> bool {
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
            // TODO: do we want to do this for the root console as well??
            ffi::TCOD_console_delete(self.con);
        }
    }
}

pub struct Map {
    tcod_map: ffi::TCOD_map_t,
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

pub struct AStarPathWithCallback<'a>{
    tcod_path: ffi::TCOD_path_t,
    // We need to keep a reference of the callback to safely dispose of it
    // when the entire struct goes away. But it's only used in the FFI, not
    // in the Rust code directly so the compiler thinks it's dead code.
    #[allow(dead_code)]
    cb: Box<|int, int, int, int|:'a -> f32>,
}

extern fn astar_path_callback(xf: c_int, yf: c_int,
                              xt: c_int, yt: c_int,
                              user_data: *mut c_void) -> c_float {
    let cb: &mut |int, int, int, int| -> f32 = unsafe { transmute(user_data) };
    (*cb)(xf as int, yf as int, xt as int, yt as int) as c_float
}

impl<'a> AStarPathWithCallback<'a> {
    pub fn new(map_width: int, map_height: int,
               path_callback: |int, int, int, int| -> f32,
               diagonal_cost: f32) -> AStarPathWithCallback {
        let user_callback = box path_callback;
        let tcod_path = unsafe {
            ffi::TCOD_path_new_using_function(map_width as c_int,
                                              map_height as c_int,
                                              Some(astar_path_callback),
                                              transmute_copy(&user_callback),
                                              diagonal_cost as c_float)
        };
        AStarPathWithCallback {
            tcod_path: tcod_path,
            cb: user_callback,
        }
    }

    pub fn find(&mut self,
                from_x: int, from_y: int,
                to_x: int, to_y: int) -> bool {
        assert!(from_x >= 0 && from_y >= 0 && to_x >= 0 && to_y >= 0);
        unsafe {
            ffi::TCOD_path_compute(self.tcod_path,
                                   from_x as c_int, from_y as c_int,
                                   to_x as c_int, to_y as c_int) != 0
        }
    }

    pub fn walk(&mut self, recalculate_when_needed: bool) -> Option<(int, int)> {
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

    pub fn reverse(&mut self) {
        unsafe {
            ffi::TCOD_path_reverse(self.tcod_path)
        }
    }

    pub fn origin(&self) -> (int, int) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_origin(self.tcod_path, &mut x, &mut y);
            (x as int, y as int)
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

    pub fn get(&self, index: int) -> Option<(int, int)> {
        if self.is_empty() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get(self.tcod_path, index as c_int, &mut x, &mut y);
            (Some((x as int, y as int)))
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
}

#[unsafe_destructor]
impl<'a> Drop for AStarPathWithCallback<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_path_delete(self.tcod_path);
        }
    }
}


pub struct AStarFromMap; //TODO

pub struct DijkstraWithCallback; //TODO

pub struct DijkstraFromMap; //TODO


#[repr(C)]
pub enum Renderer {
    GLSL = ffi::TCOD_RENDERER_GLSL as int,
    OpenGL = ffi::TCOD_RENDERER_OPENGL as int,
    SDL = ffi::TCOD_RENDERER_SDL as int,
}

#[repr(C)]
pub enum FontFlags {
    LayoutAsciiIncol = ffi::TCOD_FONT_LAYOUT_ASCII_INCOL as int,
    LayoutAsciiInrow = ffi::TCOD_FONT_LAYOUT_ASCII_INROW as int,
    TypeGreyscale = ffi::TCOD_FONT_TYPE_GREYSCALE as int,
    LayoutTcod = ffi::TCOD_FONT_LAYOUT_TCOD as int,
}

pub mod key_code {
    #[deriving(PartialEq, FromPrimitive, Show)]
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


#[deriving(PartialEq, Show)]
pub enum Key {
    Printable(char),
    Special(key_code::KeyCode),
}

pub struct KeyState {
    pub key: Key,
    pub pressed: bool,
    pub left_alt: bool,
    pub left_ctrl: bool,
    pub right_alt: bool,
    pub right_ctrl: bool,
    pub shift: bool,
}

#[deriving(PartialEq, Clone, Show)]
#[repr(C)]
pub struct Color {
    pub r: uint8_t,
    pub g: uint8_t,
    pub b: uint8_t,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color{r: red as uint8_t, g: green as uint8_t, b: blue as uint8_t}
    }
}

#[repr(C)]
pub enum TextAlignment {
    Left = ffi::TCOD_LEFT as int,
    Right = ffi::TCOD_RIGHT as int,
    Center = ffi::TCOD_CENTER as int,
}

pub mod background_flag {
    use super::ffi;

    #[repr(C)]
    pub enum BackgroundFlag {
        None = ffi::TCOD_BKGND_NONE as int,
        Set = ffi::TCOD_BKGND_SET as int,
        Multiply = ffi::TCOD_BKGND_MULTIPLY as int,
        Lighten = ffi::TCOD_BKGND_LIGHTEN as int,
        Darken = ffi::TCOD_BKGND_DARKEN as int,
        Screen = ffi::TCOD_BKGND_SCREEN as int,
        ColorDodge = ffi::TCOD_BKGND_COLOR_DODGE as int,
        ColorBurn = ffi::TCOD_BKGND_COLOR_BURN as int,
        Add = ffi::TCOD_BKGND_ADD as int,
        AddA = ffi::TCOD_BKGND_ADDA as int,
        Burn = ffi::TCOD_BKGND_BURN as int,
        Overlay = ffi::TCOD_BKGND_OVERLAY as int,
        Alph = ffi::TCOD_BKGND_ALPH as int,
        Default = ffi::TCOD_BKGND_DEFAULT as int
    }
}


pub mod system {
    use libc::{c_int};
    use ffi;

    pub fn set_fps(fps: int) {
        assert!(fps > 0);
        unsafe {
            ffi::TCOD_sys_set_fps(fps as c_int)
        }
    }

    pub fn get_fps() -> int {
        let mut result;
        unsafe {
            result = ffi::TCOD_sys_get_fps();
        }
        assert!(result >= 0);
        return result as int
    }

    pub fn get_last_frame_length() -> f32 {
        unsafe {
            ffi::TCOD_sys_get_last_frame_length() as f32
        }
    }
}


pub enum KeyPressFlag {
    Pressed = 1,
    Released = 2,
    PressedOrReleased = 1 | 2,
}
