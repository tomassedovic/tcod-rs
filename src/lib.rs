#![feature(unboxed_closures)]

extern crate libc;
extern crate "tcod-sys" as ffi;

use libc::{c_int, c_uint, c_float, uint8_t, c_void};

pub use Console::Root as RootConsole;
pub use ffi::TCOD_color_t as Color;

use std::num::FromPrimitive;
use std::ffi::CString;

#[allow(non_camel_case_types)]
type c_bool = uint8_t;


// Private wrapper over TCOD_console_t. Ideally, we'd have it as a private field
// in OffscreenConsole, but that doesn't seem to be possible now.
pub struct LibtcodConsole {
    con: ffi::TCOD_console_t,
}


impl Drop for LibtcodConsole {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_console_delete(self.con);
        }
    }
}

pub enum Console {
    Root,
    Offscreen(LibtcodConsole)
}

impl Console {
    pub fn new(width: isize, height: isize) -> Console {
        assert!(width > 0 && height > 0);
        unsafe {
            Console::Offscreen(
                LibtcodConsole{
                    con: ffi::TCOD_console_new(width as c_int, height as c_int)
                }
            )
        }
    }

    #[inline]
    fn con(&self) -> ffi::TCOD_console_t {
        match self {
            &Console::Root => 0 as ffi::TCOD_console_t,
            &Console::Offscreen(LibtcodConsole{con}) => con,
        }
    }

    pub fn init_root(width: isize, height: isize, title: &str, fullscreen: bool) -> Console {
        assert!(width > 0 && height > 0);
        unsafe {
            let c_title = CString::from_slice(title.as_bytes());
            ffi::TCOD_console_init_root(width as c_int, height as c_int,
                                            c_title.as_ptr(), fullscreen as c_bool,
                                            ffi::TCOD_RENDERER_SDL);
        }
        Console::Root
    }


    pub fn blit(source_console: &Console,
                source_x: isize, source_y: isize,
                source_width: isize, source_height: isize,
                destination_console: &mut Console,
                destination_x: isize, destination_y: isize,
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
            ffi::TCOD_console_set_key_color(self.con(), color);
        }
    }

    pub fn width(&self) -> isize {
        unsafe {
            ffi::TCOD_console_get_width(self.con()) as isize
        }
    }

    pub fn height(&self) -> isize {
        unsafe {
            ffi::TCOD_console_get_height(self.con()) as isize
        }
    }

    pub fn set_default_background(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_background(self.con(), color);
        }
    }

    pub fn set_default_foreground(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_foreground(self.con(), color);
        }
    }

    pub fn console_set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con(), color);
        }
    }

    pub fn set_char_background(&mut self, x: isize, y: isize,
                               color: Color,
                               background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_background(self.con(),
                                                  x as c_int, y as c_int,
                                                  color,
                                                  background_flag as u32)
        }
    }

    pub fn put_char(&mut self,
                    x: isize, y: isize, glyph: char,
                    background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char(self.con(),
                                       x as c_int, y as c_int, glyph as c_int,
                                       background_flag as u32);
        }
    }

    pub fn put_char_ex(&mut self,
                       x: isize, y: isize, glyph: char,
                       foreground: Color, background: Color) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char_ex(self.con(),
                                          x as c_int, y as c_int, glyph as c_int,
                                          foreground, background);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::TCOD_console_clear(self.con());
        }
    }

    pub fn print_ex(&mut self,
                    x: isize, y: isize,
                    background_flag: BackgroundFlag,
                    alignment: TextAlignment,
                    text: &str) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            let c_text = CString::from_slice(text.as_bytes());
            ffi::TCOD_console_print_ex(self.con(),
                                        x as c_int, y as c_int,
                                        background_flag as u32,
                                        alignment as u32,
                                        c_text.as_ptr());
        }
    }

    pub fn set_fade(fade: u8, fading_color: Color) {
        unsafe {
            ffi::TCOD_console_set_fade(fade, fading_color);
        }
    }

    pub fn set_custom_font(font_path: ::std::path::Path, flags: FontFlags,
                           nb_char_horizontal: isize,
                           nb_char_vertical: isize) {
        unsafe {
            let path = CString::from_slice(font_path.as_vec());
            ffi::TCOD_console_set_custom_font(
                path.as_ptr(), flags.bits() as c_int, nb_char_horizontal as c_int,
                nb_char_vertical as c_int);
        }
    }

    pub fn wait_for_keypress(flush: bool) -> KeyState {
        let tcod_key = unsafe {
            ffi::TCOD_console_wait_for_keypress(flush as c_bool)
        };
        let key = if tcod_key.vk == ffi::TCODK_CHAR {
            Key::Printable(tcod_key.c as u8 as char)
        } else {
            Key::Special(FromPrimitive::from_u32(tcod_key.vk).unwrap())
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

    pub fn check_for_keypress(status: KeyPressFlags) -> Option<KeyState> {
        let tcod_key = unsafe {
            ffi::TCOD_console_check_for_keypress(status.bits() as c_int)
        };
        if tcod_key.vk == ffi::TCODK_NONE {
            return None;
        }
        let key = if tcod_key.vk == ffi::TCODK_CHAR {
            Key::Printable(tcod_key.c as u8 as char)
        } else {
            Key::Special(FromPrimitive::from_u32(tcod_key.vk).unwrap())
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

    pub fn set_window_title(title: &str) {
        unsafe {
            let c_title = CString::from_slice(title.as_bytes());
            ffi::TCOD_console_set_window_title(c_title.as_ptr());
        }
    }
}

pub struct Map {
    tcod_map: ffi::TCOD_map_t,
}

impl Map {
    pub fn new(width: isize, height: isize) -> Map {
        assert!(width > 0 && height > 0);
        unsafe {
            Map{tcod_map: ffi::TCOD_map_new(width as c_int, height as c_int)}
        }
    }

    pub fn size(&self) -> (isize, isize) {
        unsafe {
            (ffi::TCOD_map_get_width(self.tcod_map) as isize,
             ffi::TCOD_map_get_height(self.tcod_map) as isize)
        }
    }

    pub fn set(&mut self, x: isize, y: isize, transparent: bool, walkable: bool) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_set_properties(self.tcod_map, x as c_int, y as c_int,
                                         transparent as c_bool,
                                         walkable as c_bool);
        }
    }

    pub fn is_walkable(&self, x: isize, y: isize) -> bool {
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

enum PathInnerData<'a> {
    Map(Map),
    Callback(Box<FnMut<((isize, isize), (isize, isize)), f32>+'a>, Box<(usize, usize)>),
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

pub struct AStarPath<'a>{
    tcod_path: TCODPath,
    #[allow(dead_code)]
    inner: PathInnerData<'a>,
    width: isize,
    height: isize,
}

extern "C" fn c_path_callback(xf: c_int, yf: c_int,
                          xt: c_int, yt: c_int,
                          user_data: *mut c_void) -> c_float {
    unsafe {
        let ptr: &(usize, usize) = &*(user_data as *const (usize, usize));
        let cb: &mut FnMut<((isize, isize), (isize, isize)), f32> = ::std::mem::transmute(*ptr);
        cb.call_mut(((xf as isize, yf as isize), (xt as isize, yt as isize))) as c_float
    }
}

type TcodPathCb = extern "C" fn(c_int, c_int, c_int, c_int, *mut c_void) -> c_float;

impl<'a> AStarPath<'a> {
    pub fn new_from_callback<T: 'a+FnMut((isize, isize), (isize, isize)) -> f32>(
        width: isize, height: isize, path_callback: T,
        diagonal_cost: f32) -> AStarPath<'a> {
        // Convert the closure to a trait object. This will turn it isizeo a fat pointer:
        let user_closure: Box<FnMut<((isize, isize), (isize, isize)), f32>> = Box::new(path_callback);
        unsafe {
            let fat_ptr: (usize, usize) = ::std::mem::transmute(&*user_closure);
            // Allocate the fat pointer on the heap:
            let mut ptr: Box<(usize, usize)> = Box::new(fat_ptr);
            // Create a pointer to the fat pointer. This well be passed as *void user_data:
            let user_data_ptr: *mut (usize, usize) = &mut *ptr;

            let tcod_path = ffi::TCOD_path_new_using_function(width as c_int, height as c_int,
                                                              Some(c_path_callback as TcodPathCb),
                                                              user_data_ptr as *mut c_void,
                                                              diagonal_cost as c_float);
            AStarPath {
                tcod_path: TCODPath{ptr: tcod_path},
                // Keep track of everything we've allocated on the heap. Both
                // `user_closure` and `ptr` will be deallocated when AStarPath
                // is dropped:
                inner: PathInnerData::Callback(user_closure, ptr),
                width: width,
                height: height,
            }
        }
    }

    pub fn new_from_map(map: Map, diagonal_cost: f32) -> AStarPath<'static> {
        let tcod_path = unsafe {
            ffi::TCOD_path_new_using_map(map.tcod_map, diagonal_cost as c_float)
        };
        let (w, h) = map.size();
        AStarPath {
            tcod_path: TCODPath{ptr: tcod_path},
            inner: PathInnerData::Map(map),
            width: w,
            height: h,
        }
    }

    pub fn find(&mut self,
                from: (isize, isize),
                to: (isize, isize)) -> bool {
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

    pub fn walk<'b>(&'b mut self) -> AStarPathIterator<'b> {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: false}
    }

    pub fn walk_recalculate<'b>(&'b mut self) -> AStarPathIterator<'b> {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: true}
    }

    pub fn walk_one_step(&mut self, recalculate_when_needed: bool) -> Option<(isize, isize)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path.ptr, &mut x, &mut y,
                                      recalculate_when_needed as c_bool) != 0 {
                true => Some((x as isize, y as isize)),
                false => None,
            }
        }
    }

    pub fn reverse(&mut self) {
        unsafe {
            ffi::TCOD_path_reverse(self.tcod_path.ptr)
        }
    }

    pub fn origin(&self) -> (isize, isize) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_origin(self.tcod_path.ptr, &mut x, &mut y);
            (x as isize, y as isize)
        }
    }

    pub fn destination(&self) -> (isize, isize) {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get_destination(self.tcod_path.ptr, &mut x, &mut y);
            (x as isize, y as isize)
        }
    }

    pub fn get(&self, index: isize) -> Option<(isize, isize)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get(self.tcod_path.ptr, index as c_int, &mut x, &mut y);
            (Some((x as isize, y as isize)))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_path_is_empty(self.tcod_path.ptr) != 0
        }
    }

    pub fn len(&self) -> isize {
        unsafe {
            ffi::TCOD_path_size(self.tcod_path.ptr) as isize
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

pub struct DijkstraPath<'a> {
    tcod_path: TCODDijkstraPath,
    #[allow(dead_code)]
    inner: PathInnerData<'a>,
    width: isize,
    height: isize,
}

impl<'a> DijkstraPath<'a> {
    pub fn new_from_callback<T: 'a+FnMut((isize, isize), (isize, isize)) -> f32>(
        width: isize, height: isize,
        path_callback: T,
        diagonal_cost: f32) -> DijkstraPath<'a> {
        // NOTE: this is might be a bit confusing. See the
        // AStarPath::new_from_callback implementation comments.
        let user_closure: Box<FnMut<((isize, isize), (isize, isize)), f32>> = Box::new(path_callback);
        unsafe {
            let fat_ptr: (usize, usize) = ::std::mem::transmute(&*user_closure);
            let mut ptr: Box<(usize, usize)> = Box::new(fat_ptr);
            let user_data_ptr: *mut (usize, usize) = &mut *ptr;
            let tcod_path = ffi::TCOD_dijkstra_new_using_function(width as c_int,
                                                                  height as c_int,
                                                                  Some(c_path_callback as TcodPathCb),
                                                                  user_data_ptr as *mut c_void,
                                                                  diagonal_cost as c_float);
            DijkstraPath {
                tcod_path: TCODDijkstraPath{ptr: tcod_path},
                inner: PathInnerData::Callback(user_closure, ptr),
                width: width,
                height: height,
            }
        }
    }

    pub fn new_from_map(map: Map, diagonal_cost: f32) -> DijkstraPath<'static> {
        let tcod_path = unsafe {
            ffi::TCOD_dijkstra_new(map.tcod_map, diagonal_cost as c_float)
        };
        let (w, h) = map.size();
        DijkstraPath {
            tcod_path: TCODDijkstraPath{ptr: tcod_path},
            inner: PathInnerData::Map(map),
            width: w,
            height: h,
        }
    }

    pub fn compute_grid(&mut self, root: (isize, isize)) {
        let (x, y) = root;
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            ffi::TCOD_dijkstra_compute(self.tcod_path.ptr, x as c_int, y as c_int);
        }
    }

    pub fn find(&mut self, destination: (isize, isize)) -> bool {
        let (x, y) = destination;
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            unsafe {
                ffi::TCOD_dijkstra_path_set(self.tcod_path.ptr, x as c_int, y as c_int) != 0
            }
        } else {
            false
        }
    }

    pub fn walk<'b>(&'b mut self) -> DijkstraPathIterator<'b> {
        DijkstraPathIterator{tcod_path: self.tcod_path.ptr}
    }

    pub fn walk_one_step(&mut self) -> Option<(isize, isize)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path.ptr, &mut x, &mut y) != 0 {
                true => Some((x as isize, y as isize)),
                false => None,
            }
        }
    }


    pub fn distance_from_root(&self, point: (isize, isize)) -> Option<f32> {
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

    pub fn get(&self, index: isize) -> Option<(isize, isize)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_dijkstra_get(self.tcod_path.ptr, index as c_int, &mut x, &mut y);
            Some((x as isize, y as isize))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_dijkstra_is_empty(self.tcod_path.ptr) != 0
        }
    }

    pub fn len(&self) -> isize {
        unsafe {
            ffi::TCOD_dijkstra_size(self.tcod_path.ptr) as isize
        }
    }
}

pub struct AStarPathIterator<'a> {
    tcod_path: ffi::TCOD_path_t,
    recalculate: bool,
}

impl<'a> Iterator for AStarPathIterator<'a> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<(isize, isize)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path, &mut x, &mut y,
                                      self.recalculate as c_bool) != 0 {
                true => Some((x as isize, y as isize)),
                false => None,
            }
        }
    }
}

pub struct DijkstraPathIterator<'a> {
    tcod_path: ffi::TCOD_path_t,
}

impl<'a> Iterator for DijkstraPathIterator<'a> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<(isize, isize)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path, &mut x, &mut y) != 0 {
                true => Some((x as isize, y as isize)),
                false => None,
            }
        }
    }
}


#[repr(C)]
#[derive(Copy)]
pub enum Renderer {
    GLSL = ffi::TCOD_RENDERER_GLSL as isize,
    OpenGL = ffi::TCOD_RENDERER_OPENGL as isize,
    SDL = ffi::TCOD_RENDERER_SDL as isize,
}

bitflags! {
    flags FontFlags: c_uint {
        const FONT_LAYOUT_ASCII_INCOL = ffi::TCOD_FONT_LAYOUT_ASCII_INCOL,
        const FONT_LAYOUT_ASCII_INROW = ffi::TCOD_FONT_LAYOUT_ASCII_INROW,
        const FONT_TYPE_GREYSCALE = ffi::TCOD_FONT_TYPE_GREYSCALE,
        const FONT_LAYOUT_TCOD = ffi::TCOD_FONT_LAYOUT_TCOD,
    }
}


#[derive(Copy, PartialEq, FromPrimitive, Show)]
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


#[derive(Copy, PartialEq, Show)]
pub enum Key {
    Printable(char),
    Special(KeyCode),
}

#[derive(Copy, PartialEq, Show)]
pub struct KeyState {
    pub key: Key,
    pub pressed: bool,
    pub left_alt: bool,
    pub left_ctrl: bool,
    pub right_alt: bool,
    pub right_ctrl: bool,
    pub shift: bool,
}


pub mod colors {
    pub use ffi::TCOD_black as black;
    pub use ffi::TCOD_darkest_grey as darkest_grey;
    pub use ffi::TCOD_darker_grey as darker_grey;
    pub use ffi::TCOD_dark_grey as dark_grey;
    pub use ffi::TCOD_grey as grey;
    pub use ffi::TCOD_light_grey as light_grey;
    pub use ffi::TCOD_lighter_grey as lighter_grey;
    pub use ffi::TCOD_lightest_grey as lightest_grey;
    pub use ffi::TCOD_darkest_gray as darkest_gray;
    pub use ffi::TCOD_darker_gray as darker_gray;
    pub use ffi::TCOD_dark_gray as dark_gray;
    pub use ffi::TCOD_gray as gray;
    pub use ffi::TCOD_light_gray as light_gray;
    pub use ffi::TCOD_lighter_gray as lighter_gray;
    pub use ffi::TCOD_lightest_gray as lightest_gray;
    pub use ffi::TCOD_white as white;
    pub use ffi::TCOD_darkest_sepia as darkest_sepia;
    pub use ffi::TCOD_darker_sepia as darker_sepia;
    pub use ffi::TCOD_dark_sepia as dark_sepia;
    pub use ffi::TCOD_sepia as sepia;
    pub use ffi::TCOD_light_sepia as light_sepia;
    pub use ffi::TCOD_lighter_sepia as lighter_sepia;
    pub use ffi::TCOD_lightest_sepia as lightest_sepia;
    pub use ffi::TCOD_red as red;
    pub use ffi::TCOD_flame as flame;
    pub use ffi::TCOD_orange as orange;
    pub use ffi::TCOD_amber as amber;
    pub use ffi::TCOD_yellow as yellow;
    pub use ffi::TCOD_lime as lime;
    pub use ffi::TCOD_chartreuse as chartreuse;
    pub use ffi::TCOD_green as green;
    pub use ffi::TCOD_sea as sea;
    pub use ffi::TCOD_turquoise as turquoise;
    pub use ffi::TCOD_cyan as cyan;
    pub use ffi::TCOD_sky as sky;
    pub use ffi::TCOD_azure as azure;
    pub use ffi::TCOD_blue as blue;
    pub use ffi::TCOD_han as han;
    pub use ffi::TCOD_violet as violet;
    pub use ffi::TCOD_purple as purple;
    pub use ffi::TCOD_fuchsia as fuchsia;
    pub use ffi::TCOD_magenta as magenta;
    pub use ffi::TCOD_pink as pink;
    pub use ffi::TCOD_crimson as crimson;
    pub use ffi::TCOD_dark_red as dark_red;
    pub use ffi::TCOD_dark_flame as dark_flame;
    pub use ffi::TCOD_dark_orange as dark_orange;
    pub use ffi::TCOD_dark_amber as dark_amber;
    pub use ffi::TCOD_dark_yellow as dark_yellow;
    pub use ffi::TCOD_dark_lime as dark_lime;
    pub use ffi::TCOD_dark_chartreuse as dark_chartreuse;
    pub use ffi::TCOD_dark_green as dark_green;
    pub use ffi::TCOD_dark_sea as dark_sea;
    pub use ffi::TCOD_dark_turquoise as dark_turquoise;
    pub use ffi::TCOD_dark_cyan as dark_cyan;
    pub use ffi::TCOD_dark_sky as dark_sky;
    pub use ffi::TCOD_dark_azure as dark_azure;
    pub use ffi::TCOD_dark_blue as dark_blue;
    pub use ffi::TCOD_dark_han as dark_han;
    pub use ffi::TCOD_dark_violet as dark_violet;
    pub use ffi::TCOD_dark_purple as dark_purple;
    pub use ffi::TCOD_dark_fuchsia as dark_fuchsia;
    pub use ffi::TCOD_dark_magenta as dark_magenta;
    pub use ffi::TCOD_dark_pink as dark_pink;
    pub use ffi::TCOD_dark_crimson as dark_crimson;
    pub use ffi::TCOD_darker_red as darker_red;
    pub use ffi::TCOD_darker_flame as darker_flame;
    pub use ffi::TCOD_darker_orange as darker_orange;
    pub use ffi::TCOD_darker_amber as darker_amber;
    pub use ffi::TCOD_darker_yellow as darker_yellow;
    pub use ffi::TCOD_darker_lime as darker_lime;
    pub use ffi::TCOD_darker_chartreuse as darker_chartreuse;
    pub use ffi::TCOD_darker_green as darker_green;
    pub use ffi::TCOD_darker_sea as darker_sea;
    pub use ffi::TCOD_darker_turquoise as darker_turquoise;
    pub use ffi::TCOD_darker_cyan as darker_cyan;
    pub use ffi::TCOD_darker_sky as darker_sky;
    pub use ffi::TCOD_darker_azure as darker_azure;
    pub use ffi::TCOD_darker_blue as darker_blue;
    pub use ffi::TCOD_darker_han as darker_han;
    pub use ffi::TCOD_darker_violet as darker_violet;
    pub use ffi::TCOD_darker_purple as darker_purple;
    pub use ffi::TCOD_darker_fuchsia as darker_fuchsia;
    pub use ffi::TCOD_darker_magenta as darker_magenta;
    pub use ffi::TCOD_darker_pink as darker_pink;
    pub use ffi::TCOD_darker_crimson as darker_crimson;
    pub use ffi::TCOD_darkest_red as darkest_red;
    pub use ffi::TCOD_darkest_flame as darkest_flame;
    pub use ffi::TCOD_darkest_orange as darkest_orange;
    pub use ffi::TCOD_darkest_amber as darkest_amber;
    pub use ffi::TCOD_darkest_yellow as darkest_yellow;
    pub use ffi::TCOD_darkest_lime as darkest_lime;
    pub use ffi::TCOD_darkest_chartreuse as darkest_chartreuse;
    pub use ffi::TCOD_darkest_green as darkest_green;
    pub use ffi::TCOD_darkest_sea as darkest_sea;
    pub use ffi::TCOD_darkest_turquoise as darkest_turquoise;
    pub use ffi::TCOD_darkest_cyan as darkest_cyan;
    pub use ffi::TCOD_darkest_sky as darkest_sky;
    pub use ffi::TCOD_darkest_azure as darkest_azure;
    pub use ffi::TCOD_darkest_blue as darkest_blue;
    pub use ffi::TCOD_darkest_han as darkest_han;
    pub use ffi::TCOD_darkest_violet as darkest_violet;
    pub use ffi::TCOD_darkest_purple as darkest_purple;
    pub use ffi::TCOD_darkest_fuchsia as darkest_fuchsia;
    pub use ffi::TCOD_darkest_magenta as darkest_magenta;
    pub use ffi::TCOD_darkest_pink as darkest_pink;
    pub use ffi::TCOD_darkest_crimson as darkest_crimson;
    pub use ffi::TCOD_light_red as light_red;
    pub use ffi::TCOD_light_flame as light_flame;
    pub use ffi::TCOD_light_orange as light_orange;
    pub use ffi::TCOD_light_amber as light_amber;
    pub use ffi::TCOD_light_yellow as light_yellow;
    pub use ffi::TCOD_light_lime as light_lime;
    pub use ffi::TCOD_light_chartreuse as light_chartreuse;
    pub use ffi::TCOD_light_green as light_green;
    pub use ffi::TCOD_light_sea as light_sea;
    pub use ffi::TCOD_light_turquoise as light_turquoise;
    pub use ffi::TCOD_light_cyan as light_cyan;
    pub use ffi::TCOD_light_sky as light_sky;
    pub use ffi::TCOD_light_azure as light_azure;
    pub use ffi::TCOD_light_blue as light_blue;
    pub use ffi::TCOD_light_han as light_han;
    pub use ffi::TCOD_light_violet as light_violet;
    pub use ffi::TCOD_light_purple as light_purple;
    pub use ffi::TCOD_light_fuchsia as light_fuchsia;
    pub use ffi::TCOD_light_magenta as light_magenta;
    pub use ffi::TCOD_light_pink as light_pink;
    pub use ffi::TCOD_light_crimson as light_crimson;
    pub use ffi::TCOD_lighter_red as lighter_red;
    pub use ffi::TCOD_lighter_flame as lighter_flame;
    pub use ffi::TCOD_lighter_orange as lighter_orange;
    pub use ffi::TCOD_lighter_amber as lighter_amber;
    pub use ffi::TCOD_lighter_yellow as lighter_yellow;
    pub use ffi::TCOD_lighter_lime as lighter_lime;
    pub use ffi::TCOD_lighter_chartreuse as lighter_chartreuse;
    pub use ffi::TCOD_lighter_green as lighter_green;
    pub use ffi::TCOD_lighter_sea as lighter_sea;
    pub use ffi::TCOD_lighter_turquoise as lighter_turquoise;
    pub use ffi::TCOD_lighter_cyan as lighter_cyan;
    pub use ffi::TCOD_lighter_sky as lighter_sky;
    pub use ffi::TCOD_lighter_azure as lighter_azure;
    pub use ffi::TCOD_lighter_blue as lighter_blue;
    pub use ffi::TCOD_lighter_han as lighter_han;
    pub use ffi::TCOD_lighter_violet as lighter_violet;
    pub use ffi::TCOD_lighter_purple as lighter_purple;
    pub use ffi::TCOD_lighter_fuchsia as lighter_fuchsia;
    pub use ffi::TCOD_lighter_magenta as lighter_magenta;
    pub use ffi::TCOD_lighter_pink as lighter_pink;
    pub use ffi::TCOD_lighter_crimson as lighter_crimson;
    pub use ffi::TCOD_lightest_red as lightest_red;
    pub use ffi::TCOD_lightest_flame as lightest_flame;
    pub use ffi::TCOD_lightest_orange as lightest_orange;
    pub use ffi::TCOD_lightest_amber as lightest_amber;
    pub use ffi::TCOD_lightest_yellow as lightest_yellow;
    pub use ffi::TCOD_lightest_lime as lightest_lime;
    pub use ffi::TCOD_lightest_chartreuse as lightest_chartreuse;
    pub use ffi::TCOD_lightest_green as lightest_green;
    pub use ffi::TCOD_lightest_sea as lightest_sea;
    pub use ffi::TCOD_lightest_turquoise as lightest_turquoise;
    pub use ffi::TCOD_lightest_cyan as lightest_cyan;
    pub use ffi::TCOD_lightest_sky as lightest_sky;
    pub use ffi::TCOD_lightest_azure as lightest_azure;
    pub use ffi::TCOD_lightest_blue as lightest_blue;
    pub use ffi::TCOD_lightest_han as lightest_han;
    pub use ffi::TCOD_lightest_violet as lightest_violet;
    pub use ffi::TCOD_lightest_purple as lightest_purple;
    pub use ffi::TCOD_lightest_fuchsia as lightest_fuchsia;
    pub use ffi::TCOD_lightest_magenta as lightest_magenta;
    pub use ffi::TCOD_lightest_pink as lightest_pink;
    pub use ffi::TCOD_lightest_crimson as lightest_crimson;
    pub use ffi::TCOD_desaturated_red as desaturated_red;
    pub use ffi::TCOD_desaturated_flame as desaturated_flame;
    pub use ffi::TCOD_desaturated_orange as desaturated_orange;
    pub use ffi::TCOD_desaturated_amber as desaturated_amber;
    pub use ffi::TCOD_desaturated_yellow as desaturated_yellow;
    pub use ffi::TCOD_desaturated_lime as desaturated_lime;
    pub use ffi::TCOD_desaturated_chartreuse as desaturated_chartreuse;
    pub use ffi::TCOD_desaturated_green as desaturated_green;
    pub use ffi::TCOD_desaturated_sea as desaturated_sea;
    pub use ffi::TCOD_desaturated_turquoise as desaturated_turquoise;
    pub use ffi::TCOD_desaturated_cyan as desaturated_cyan;
    pub use ffi::TCOD_desaturated_sky as desaturated_sky;
    pub use ffi::TCOD_desaturated_azure as desaturated_azure;
    pub use ffi::TCOD_desaturated_blue as desaturated_blue;
    pub use ffi::TCOD_desaturated_han as desaturated_han;
    pub use ffi::TCOD_desaturated_violet as desaturated_violet;
    pub use ffi::TCOD_desaturated_purple as desaturated_purple;
    pub use ffi::TCOD_desaturated_fuchsia as desaturated_fuchsia;
    pub use ffi::TCOD_desaturated_magenta as desaturated_magenta;
    pub use ffi::TCOD_desaturated_pink as desaturated_pink;
    pub use ffi::TCOD_desaturated_crimson as desaturated_crimson;
    pub use ffi::TCOD_brass as brass;
    pub use ffi::TCOD_copper as copper;
    pub use ffi::TCOD_gold as gold;
    pub use ffi::TCOD_silver as silver;
    pub use ffi::TCOD_celadon as celadon;
    pub use ffi::TCOD_peach as peach;
}

#[repr(C)]
#[derive(Copy)]
pub enum TextAlignment {
    Left = ffi::TCOD_LEFT as isize,
    Right = ffi::TCOD_RIGHT as isize,
    Center = ffi::TCOD_CENTER as isize,
}


#[repr(C)]
#[derive(Copy)]
pub enum BackgroundFlag {
    None = ffi::TCOD_BKGND_NONE as isize,
    Set = ffi::TCOD_BKGND_SET as isize,
    Multiply = ffi::TCOD_BKGND_MULTIPLY as isize,
    Lighten = ffi::TCOD_BKGND_LIGHTEN as isize,
    Darken = ffi::TCOD_BKGND_DARKEN as isize,
    Screen = ffi::TCOD_BKGND_SCREEN as isize,
    ColorDodge = ffi::TCOD_BKGND_COLOR_DODGE as isize,
    ColorBurn = ffi::TCOD_BKGND_COLOR_BURN as isize,
    Add = ffi::TCOD_BKGND_ADD as isize,
    AddA = ffi::TCOD_BKGND_ADDA as isize,
    Burn = ffi::TCOD_BKGND_BURN as isize,
    Overlay = ffi::TCOD_BKGND_OVERLAY as isize,
    Alph = ffi::TCOD_BKGND_ALPH as isize,
    Default = ffi::TCOD_BKGND_DEFAULT as isize
}


pub mod system {
    use libc::{c_int};
    use ffi;

    pub fn set_fps(fps: isize) {
        assert!(fps > 0);
        unsafe {
            ffi::TCOD_sys_set_fps(fps as c_int)
        }
    }

    pub fn get_fps() -> isize {
        let mut result;
        unsafe {
            result = ffi::TCOD_sys_get_fps();
        }
        assert!(result >= 0);
        return result as isize
    }

    pub fn get_last_frame_length() -> f32 {
        unsafe {
            ffi::TCOD_sys_get_last_frame_length() as f32
        }
    }
}


bitflags! {
    flags KeyPressFlags: c_uint {
        const KEY_PRESSED = ffi::TCOD_KEY_PRESSED,
        const KEY_RELEASED = ffi::TCOD_KEY_RELEASED,
    }
}
