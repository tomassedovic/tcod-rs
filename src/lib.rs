#![feature(globs, unboxed_closures)]

extern crate libc;

use std::mem::transmute;

use libc::{c_int, c_float, uint8_t, c_void};

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod ffi;

#[allow(non_camel_case_types)]
type c_bool = uint8_t;


// Private wrapper over TCOD_console_t. Ideally, we'd have it as a private field
// in OffscreenConsole, but that doesn't seem to be possible now.
pub struct LibtcodConsole {
    con: ffi::TCOD_console_t,
}

pub enum Console {
    RootConsole,
    OffscreenConsole(LibtcodConsole)
}

impl Console {
    pub fn new(width: int, height: int) -> Console {
        assert!(width > 0 && height > 0);
        unsafe {
            OffscreenConsole(
                LibtcodConsole{
                    con: ffi::TCOD_console_new(width as c_int, height as c_int)
                }
            )
        }
    }

    #[inline]
    fn con(&self) -> ffi::TCOD_console_t {
        match self {
            &RootConsole => 0 as ffi::TCOD_console_t,
            &OffscreenConsole(LibtcodConsole{con}) => con,
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
        RootConsole
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
            ffi::TCOD_console_blit(source_console.con(),
                                   source_x as c_int, source_y as c_int,
                                   source_width as c_int, source_height as c_int,
                                   destination_console.con(),
                                   destination_x as c_int, destination_y as c_int,
                                   foreground_alpha as c_float,
                                   background_alpha as c_float)
        }
    }

    pub fn set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con(), transmute(color));
        }
    }

    pub fn width(&self) -> int {
        unsafe {
            ffi::TCOD_console_get_width(self.con()) as int
        }
    }

    pub fn height(&self) -> int {
        unsafe {
            ffi::TCOD_console_get_height(self.con()) as int
        }
    }

    pub fn set_default_background(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_background(self.con(), transmute(color));
        }
    }

    pub fn set_default_foreground(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_foreground(self.con(), transmute(color));
        }
    }

    pub fn console_set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con(), transmute(color));
        }
    }

    pub fn set_char_background(&mut self, x: int, y: int,
                               color: Color,
                               background_flag: background_flag::BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_background(self.con(),
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
            ffi::TCOD_console_put_char(self.con(),
                                       x as c_int, y as c_int, glyph as c_int,
                                       background_flag as u32);
        }
    }

    pub fn put_char_ex(&mut self,
                       x: int, y: int, glyph: char,
                       foreground: Color, background: Color) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char_ex(self.con(),
                                          x as c_int, y as c_int, glyph as c_int,
                                          transmute(foreground),
                                          transmute(background));
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::TCOD_console_clear(self.con());
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
                ffi::TCOD_console_print_ex(self.con(),
                                           x as c_int, y as c_int,
                                           background_flag as u32,
                                           alignment as u32,
                                           c_text));
        }
    }

    pub fn set_fade(fade: u8, fading_color: Color) {
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

    pub fn wait_for_keypress(flush: bool) -> KeyState {
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

    pub fn check_for_keypress(status: KeyPressFlag) -> Option<KeyState> {
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

    pub fn flush() {
        unsafe {
            ffi::TCOD_console_flush();
        }
    }
}

impl Drop for Console {
    fn drop(&mut self) {
        match *self {
            RootConsole => (),
            OffscreenConsole(LibtcodConsole{con}) => unsafe {
                ffi::TCOD_console_delete(con);
            }
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

enum PathInnerData {
    PathMap(Map),
    PathCallback(Box<FnMut<((int, int), (int, int)), f32>+'static>, Box<(uint, uint)>),
}

// We need to wrap the pointer in a struct so that we can implement a
// destructor. But we can't put ffi::TCOD_path_t directly inside AStarPath
// because it includes the boxed closure which has a 'static lifetime so we
// can't attach a destructor to it.
//
// Unless we use #[unsafe_destructor] and I've no idea what could go wrong there.
struct TCODPath {
    ptr: ffi::TCOD_path_t,
}

impl Drop for TCODPath {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_path_delete(self.ptr);
        }
    }
}

pub struct AStarPath{
    tcod_path: TCODPath,
    #[allow(dead_code)]
    inner: PathInnerData,
    width: int,
    height: int,
}

extern "C" fn c_path_callback(xf: c_int, yf: c_int,
                          xt: c_int, yt: c_int,
                          user_data: *mut c_void) -> c_float {
    unsafe {
        let ptr: &(uint, uint) = &*(user_data as *const (uint, uint));
        let cb: &mut FnMut<((int, int), (int, int)), f32> = transmute(*ptr);
        cb.call_mut(((xf as int, yf as int), (xt as int, yt as int))) as c_float
    }
}

impl AStarPath {
    pub fn new_from_callback<T: 'static+FnMut(from: (int, int), to: (int, int)) -> f32>(
        width: int, height: int, path_callback: T,
        diagonal_cost: f32) -> AStarPath {
        // Convert the closure to a trait object. This will turn it into a fat pointer:
        let user_closure: Box<FnMut<((int, int), (int, int)), f32>> = box path_callback;
        unsafe {
            let fat_ptr: (uint, uint) = transmute(&*user_closure);
            // Allocate the fat pointer on the heap:
            let mut ptr: Box<(uint, uint)> = box fat_ptr;
            // Create a pointer to the fat pointer. This well be passed as *void user_data:
            let user_data_ptr: *mut (uint, uint) = &mut *ptr;

            let tcod_path = ffi::TCOD_path_new_using_function(width as c_int, height as c_int,
                                                              Some(c_path_callback),
                                                              user_data_ptr as *mut c_void,
                                                              diagonal_cost as c_float);
            AStarPath {
                tcod_path: TCODPath{ptr: tcod_path},
                // Keep track of everything we've allocated on the heap. Both
                // `user_closure` and `ptr` will be deallocated when AStarPath
                // is dropped:
                inner: PathCallback(user_closure, ptr),
                width: width,
                height: height,
            }
        }
    }

    pub fn new_from_map(map: Map, diagonal_cost: f32) -> AStarPath {
        let tcod_path = unsafe {
            ffi::TCOD_path_new_using_map(map.tcod_map, diagonal_cost as c_float)
        };
        let (w, h) = map.size();
        AStarPath {
            tcod_path: TCODPath{ptr: tcod_path},
            inner: PathMap(map),
            width: w,
            height: h,
        }
    }

    pub fn find(&mut self,
                from: (int, int),
                to: (int, int)) -> bool {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        assert!(from_x >= 0 && from_y >= 0 && to_x >= 0 && to_y >= 0);
        assert!(from_x < self.width && from_y < self.height && to_x < self.width && to_y < self.height);
        unsafe {
            ffi::TCOD_path_compute(self.tcod_path.ptr,
                                   from_x as c_int, from_y as c_int,
                                   to_x as c_int, to_y as c_int) != 0
        }
    }

    pub fn walk<'a>(&'a mut self) -> AStarPathIterator<'a> {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: false}
    }

    pub fn walk_recalculate<'a>(&'a mut self) -> AStarPathIterator<'a> {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: true}
    }

    pub fn walk_one_step(&mut self, recalculate_when_needed: bool) -> Option<(int, int)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path.ptr, &mut x, &mut y,
                                      recalculate_when_needed as c_bool) != 0 {
                true => Some((x as int, y as int)),
                false => None,
            }
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            ffi::TCOD_path_reverse(self.tcod_path.ptr)
        }
    }

    pub fn origin(&self) -> (int, int) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_origin(self.tcod_path.ptr, &mut x, &mut y);
            (x as int, y as int)
        }
    }

    pub fn destination(&self) -> (int, int) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_destination(self.tcod_path.ptr, &mut x, &mut y);
            (x as int, y as int)
        }
    }

    pub fn get(&self, index: int) -> Option<(int, int)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get(self.tcod_path.ptr, index as c_int, &mut x, &mut y);
            (Some((x as int, y as int)))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_path_is_empty(self.tcod_path.ptr) != 0
        }
    }

    pub fn len(&self) -> int {
        unsafe {
            ffi::TCOD_path_size(self.tcod_path.ptr) as int
        }
    }
}

struct TCODDijkstraPath {
    ptr: ffi::TCOD_dijkstra_t,
}

impl Drop for TCODDijkstraPath {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_dijkstra_delete(self.ptr);
        }
    }
}

pub struct DijkstraPath {
    tcod_path: TCODDijkstraPath,
    #[allow(dead_code)]
    inner: PathInnerData,
    width: int,
    height: int,
}

impl DijkstraPath {
    pub fn new_from_callback<T: 'static+FnMut((int, int), (int, int)) -> f32>(
        width: int, height: int,
        path_callback: T,
        diagonal_cost: f32) -> DijkstraPath {
        // NOTE: this is might be a bit confusing. See the
        // AStarPath::new_from_callback implementation comments.
        let user_closure: Box<FnMut<((int, int), (int, int)), f32>> = box path_callback;
        unsafe {
            let fat_ptr: (uint, uint) = transmute(&*user_closure);
            let mut ptr: Box<(uint, uint)> = box fat_ptr;
            let user_data_ptr: *mut (uint, uint) = &mut *ptr;
            let tcod_path = ffi::TCOD_dijkstra_new_using_function(width as c_int,
                                                                  height as c_int,
                                                                  Some(c_path_callback),
                                                                  user_data_ptr as *mut c_void,
                                                                  diagonal_cost as c_float);
            DijkstraPath {
                tcod_path: TCODDijkstraPath{ptr: tcod_path},
                inner: PathCallback(user_closure, ptr),
                width: width,
                height: height,
            }
        }
    }

    pub fn new_from_map(map: Map, diagonal_cost: f32) -> DijkstraPath {
        let tcod_path = unsafe {
            ffi::TCOD_dijkstra_new(map.tcod_map, diagonal_cost as c_float)
        };
        let (w, h) = map.size();
        DijkstraPath {
            tcod_path: TCODDijkstraPath{ptr: tcod_path},
            inner: PathMap(map),
            width: w,
            height: h,
        }
    }

    pub fn compute_grid(&mut self, root: (int, int)) {
        let (x, y) = root;
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            ffi::TCOD_dijkstra_compute(self.tcod_path.ptr, x as c_int, y as c_int);
        }
    }

    pub fn find(&mut self, destination: (int, int)) -> bool {
        let (x, y) = destination;
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            unsafe {
                ffi::TCOD_dijkstra_path_set(self.tcod_path.ptr, x as c_int, y as c_int) != 0
            }
        } else {
            false
        }
    }

    pub fn walk<'a>(&'a mut self) -> DijkstraPathIterator<'a> {
        DijkstraPathIterator{tcod_path: self.tcod_path.ptr}
    }

    pub fn walk_one_step(&mut self) -> Option<(int, int)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path.ptr, &mut x, &mut y) != 0 {
                true => Some((x as int, y as int)),
                false => None,
            }
        }
    }


    pub fn distance_from_root(&self, point: (int, int)) -> Option<f32> {
        let (x, y) = point;
        let result = unsafe {
            ffi::TCOD_dijkstra_get_distance(self.tcod_path.ptr, x as c_int, y as c_int)
        };
        if result == -1.0 {
            None
        } else {
            Some(result as f32)
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            ffi::TCOD_dijkstra_reverse(self.tcod_path.ptr);
        }
    }

    pub fn get(&self, index: int) -> Option<(int, int)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_dijkstra_get(self.tcod_path.ptr, index as c_int, &mut x, &mut y);
            Some((x as int, y as int))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_dijkstra_is_empty(self.tcod_path.ptr) != 0
        }
    }

    pub fn len(&self) -> int {
        unsafe {
            ffi::TCOD_dijkstra_size(self.tcod_path.ptr) as int
        }
    }
}

pub struct AStarPathIterator<'a> {
    tcod_path: ffi::TCOD_path_t,
    recalculate: bool,
}

impl<'a> Iterator<(int, int)> for AStarPathIterator<'a> {
    fn next(&mut self) -> Option<(int, int)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path, &mut x, &mut y,
                                      self.recalculate as c_bool) != 0 {
                true => Some((x as int, y as int)),
                false => None,
            }
        }
    }
}

pub struct DijkstraPathIterator<'a> {
    tcod_path: ffi::TCOD_path_t,
}

impl<'a> Iterator<(int, int)> for DijkstraPathIterator<'a> {
    fn next(&mut self) -> Option<(int, int)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path, &mut x, &mut y) != 0 {
                true => Some((x as int, y as int)),
                false => None,
            }
        }
    }
}


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

#[deriving(PartialEq, Show)]
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
