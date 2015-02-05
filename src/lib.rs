#![feature(libc, std_misc, path, core, hash, io)]

extern crate libc;
extern crate "tcod-sys" as ffi;
#[macro_use] extern crate bitflags;

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
    pub fn new(width: i32, height: i32) -> Console {
        assert!(width > 0 && height > 0);
        unsafe {
            Console::Offscreen(
                LibtcodConsole{
                    con: ffi::TCOD_console_new(width, height)
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

    pub fn init_root(width: i32, height: i32, title: &str, fullscreen: bool) -> Console {
        assert!(width > 0 && height > 0);
        unsafe {
            let c_title = CString::from_slice(title.as_bytes());
            ffi::TCOD_console_init_root(width, height,
                                            c_title.as_ptr(), fullscreen as c_bool,
                                            ffi::TCOD_RENDERER_SDL);
        }
        Console::Root
    }


    pub fn blit(source_console: &Console,
                source_x: i32, source_y: i32,
                source_width: i32, source_height: i32,
                destination_console: &mut Console,
                destination_x: i32, destination_y: i32,
                foreground_alpha: f32, background_alpha: f32) {
        assert!(source_x >= 0 && source_y >= 0 &&
                source_width > 0 && source_height > 0 &&
                destination_x >= 0 && destination_y >= 0);
        unsafe {
            ffi::TCOD_console_blit(source_console.con(),
                                   source_x, source_y,
                                   source_width, source_height,
                                   destination_console.con(),
                                   destination_x, destination_y,
                                   foreground_alpha as c_float,
                                   background_alpha as c_float)
        }
    }

    pub fn set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con(), color);
        }
    }

    pub fn width(&self) -> i32 {
        unsafe {
            ffi::TCOD_console_get_width(self.con())
        }
    }

    pub fn height(&self) -> i32 {
        unsafe {
            ffi::TCOD_console_get_height(self.con())
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

    pub fn set_char_background(&mut self, x: i32, y: i32,
                               color: Color,
                               background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_background(self.con(),
                                                  x, y,
                                                  color,
                                                  background_flag as u32)
        }
    }

    pub fn put_char(&mut self,
                    x: i32, y: i32, glyph: char,
                    background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char(self.con(),
                                       x, y, glyph as i32,
                                       background_flag as u32);
        }
    }

    pub fn put_char_ex(&mut self,
                       x: i32, y: i32, glyph: char,
                       foreground: Color, background: Color) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char_ex(self.con(),
                                          x, y, glyph as i32,
                                          foreground, background);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::TCOD_console_clear(self.con());
        }
    }

    pub fn print_ex(&mut self,
                    x: i32, y: i32,
                    background_flag: BackgroundFlag,
                    alignment: TextAlignment,
                    text: &str) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            let c_text = CString::from_slice(text.as_bytes());
            ffi::TCOD_console_print_ex(self.con(),
                                        x, y,
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

    pub fn set_custom_font(font_path: ::std::old_path::Path, flags: FontFlags,
                           nb_char_horizontal: i32,
                           nb_char_vertical: i32) {
        unsafe {
            let path = CString::from_slice(font_path.as_vec());
            ffi::TCOD_console_set_custom_font(
                path.as_ptr(), flags.bits() as i32, nb_char_horizontal,
                nb_char_vertical);
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
            ffi::TCOD_console_check_for_keypress(status.bits() as i32)
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
    pub fn new(width: i32, height: i32) -> Map {
        assert!(width > 0 && height > 0);
        unsafe {
            Map{tcod_map: ffi::TCOD_map_new(width, height)}
        }
    }

    pub fn size(&self) -> (i32, i32) {
        unsafe {
            (ffi::TCOD_map_get_width(self.tcod_map),
             ffi::TCOD_map_get_height(self.tcod_map))
        }
    }

    pub fn set(&mut self, x: i32, y: i32, transparent: bool, walkable: bool) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_set_properties(self.tcod_map, x, y,
                                         transparent as c_bool,
                                         walkable as c_bool);
        }
    }

    pub fn compute_fov(&mut self, origin_x: i32, origin_y: i32, max_radius: i32,
                       light_walls: bool, algo: FovAlgorithm) {
        assert!(origin_x >= 0 && origin_y >= 0);
        unsafe {
            ffi::TCOD_map_compute_fov(self.tcod_map, origin_x, origin_y, max_radius,
                                     light_walls as c_bool,
                                     algo as u32);
        }
    }

    pub fn is_in_fov(&self, x: i32, y: i32) -> bool {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_is_in_fov(self.tcod_map, x, y) != 0
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_map_is_walkable(self.tcod_map, x, y) != 0
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
    Callback(Box<FnMut((i32, i32), (i32, i32)) -> f32+'a>, Box<(usize, usize)>),
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
    width: i32,
    height: i32,
}

extern "C" fn c_path_callback(xf: c_int, yf: c_int,
                          xt: c_int, yt: c_int,
                          user_data: *mut c_void) -> c_float {
    unsafe {
        let ptr: &(usize, usize) = &*(user_data as *const (usize, usize));
        let cb: &mut FnMut((i32, i32), (i32, i32)) -> f32 = ::std::mem::transmute(*ptr);
        //cb.call_mut(((xf, yf), (xt, yt))) as c_float
        cb((xf, yf), (xt, yt)) as c_float
    }
}

type TcodPathCb = extern "C" fn(c_int, c_int, c_int, c_int, *mut c_void) -> c_float;

impl<'a> AStarPath<'a> {
    pub fn new_from_callback<T: 'a+FnMut((i32, i32), (i32, i32)) -> f32>(
        width: i32, height: i32, path_callback: T,
        diagonal_cost: f32) -> AStarPath<'a> {
        // Convert the closure to a trait object. This will turn it into a fat pointer:
        let user_closure: Box<FnMut((i32, i32), (i32, i32)) -> f32> = Box::new(path_callback);
        unsafe {
            let fat_ptr: (usize, usize) = ::std::mem::transmute(&*user_closure);
            // Allocate the fat pointer on the heap:
            let mut ptr: Box<(usize, usize)> = Box::new(fat_ptr);
            // Create a pointer to the fat pointer. This well be passed as *void user_data:
            let user_data_ptr: *mut (usize, usize) = &mut *ptr;

            let tcod_path = ffi::TCOD_path_new_using_function(width, height,
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
                from: (i32, i32),
                to: (i32, i32)) -> bool {
        let (from_x, from_y) = from;
        let (to_x, to_y) = to;
        assert!(from_x >= 0 && from_y >= 0 && to_x >= 0 && to_y >= 0);
        assert!(from_x < self.width && from_y < self.height && to_x < self.width && to_y < self.height);
        unsafe {
            ffi::TCOD_path_compute(self.tcod_path.ptr,
                                   from_x, from_y,
                                   to_x, to_y) != 0
        }
    }

    pub fn walk<'b>(&'b mut self) -> AStarPathIterator<'b> {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: false}
    }

    pub fn walk_recalculate<'b>(&'b mut self) -> AStarPathIterator<'b> {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: true}
    }

    pub fn walk_one_step(&mut self, recalculate_when_needed: bool) -> Option<(i32, i32)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_path_walk(self.tcod_path.ptr, &mut x, &mut y,
                                      recalculate_when_needed as c_bool) != 0 {
                true => Some((x, y)),
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

    pub fn get(&self, index: i32) -> Option<(i32, i32)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_path_get(self.tcod_path.ptr, index, &mut x, &mut y);
            (Some((x, y)))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_path_is_empty(self.tcod_path.ptr) != 0
        }
    }

    pub fn len(&self) -> i32 {
        unsafe {
            ffi::TCOD_path_size(self.tcod_path.ptr) as i32
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
    width: i32,
    height: i32,
}

impl<'a> DijkstraPath<'a> {
    pub fn new_from_callback<T: 'a+FnMut((i32, i32), (i32, i32)) -> f32>(
        width: i32, height: i32,
        path_callback: T,
        diagonal_cost: f32) -> DijkstraPath<'a> {
        // NOTE: this is might be a bit confusing. See the
        // AStarPath::new_from_callback implementation comments.
        let user_closure: Box<FnMut((i32, i32), (i32, i32)) -> f32> = Box::new(path_callback);
        unsafe {
            let fat_ptr: (usize, usize) = ::std::mem::transmute(&*user_closure);
            let mut ptr: Box<(usize, usize)> = Box::new(fat_ptr);
            let user_data_ptr: *mut (usize, usize) = &mut *ptr;
            let tcod_path = ffi::TCOD_dijkstra_new_using_function(width,
                                                                  height,
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

    pub fn compute_grid(&mut self, root: (i32, i32)) {
        let (x, y) = root;
        assert!(x >= 0 && y >= 0 && x < self.width && y < self.height);
        unsafe {
            ffi::TCOD_dijkstra_compute(self.tcod_path.ptr, x, y);
        }
    }

    pub fn find(&mut self, destination: (i32, i32)) -> bool {
        let (x, y) = destination;
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            unsafe {
                ffi::TCOD_dijkstra_path_set(self.tcod_path.ptr, x, y) != 0
            }
        } else {
            false
        }
    }

    pub fn walk<'b>(&'b mut self) -> DijkstraPathIterator<'b> {
        DijkstraPathIterator{tcod_path: self.tcod_path.ptr}
    }

    pub fn walk_one_step(&mut self) -> Option<(i32, i32)> {
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            match ffi::TCOD_dijkstra_path_walk(self.tcod_path.ptr, &mut x, &mut y) != 0 {
                true => Some((x, y)),
                false => None,
            }
        }
    }


    pub fn distance_from_root(&self, point: (i32, i32)) -> Option<f32> {
        let (x, y) = point;
        let result = unsafe {
            ffi::TCOD_dijkstra_get_distance(self.tcod_path.ptr, x, y)
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

    pub fn get(&self, index: i32) -> Option<(i32, i32)> {
        if index < 0 || index >= self.len() {
            return None;
        }
        unsafe {
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            ffi::TCOD_dijkstra_get(self.tcod_path.ptr, index, &mut x, &mut y);
            Some((x, y))
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            ffi::TCOD_dijkstra_is_empty(self.tcod_path.ptr) != 0
        }
    }

    pub fn len(&self) -> i32 {
        unsafe {
            ffi::TCOD_dijkstra_size(self.tcod_path.ptr) as i32
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


#[derive(Copy, PartialEq, FromPrimitive, Debug)]
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


#[derive(Copy, PartialEq, Debug)]
pub enum Key {
    Printable(char),
    Special(KeyCode),
}

#[derive(Copy, PartialEq, Debug)]
pub struct KeyState {
    pub key: Key,
    pub pressed: bool,
    pub left_alt: bool,
    pub left_ctrl: bool,
    pub right_alt: bool,
    pub right_ctrl: bool,
    pub shift: bool,
}

#[derive(Copy, PartialEq, Debug)]
pub struct MouseState {
    pub x: isize,
    pub y: isize,
    pub dx: isize,
    pub dy: isize,
    pub cx: isize,
    pub cy: isize,
    pub dcx: isize,
    pub dcy: isize,
    pub lbutton: bool,
    pub rbutton: bool,
    pub mbutton: bool,
    pub lbutton_pressed: bool,
    pub rbutton_pressed: bool,
    pub mbutton_pressed: bool,
    pub wheel_up: bool,
    pub wheel_down: bool,
}

#[repr(C)]
#[derive(Copy, Debug)]
pub enum FovAlgorithm {
    Basic       = ffi::FOV_BASIC as isize,
    Diamond     = ffi::FOV_DIAMOND as isize,
    Shadow      = ffi::FOV_SHADOW as isize,
    Permissive0 = ffi::FOV_PERMISSIVE_0 as isize,
    Permissive1 = ffi::FOV_PERMISSIVE_1 as isize,
    Permissive2 = ffi::FOV_PERMISSIVE_2 as isize,
    Permissive3 = ffi::FOV_PERMISSIVE_3 as isize,
    Permissive4 = ffi::FOV_PERMISSIVE_4 as isize,
    Permissive5 = ffi::FOV_PERMISSIVE_5 as isize,
    Permissive6 = ffi::FOV_PERMISSIVE_6 as isize,
    Permissive7 = ffi::FOV_PERMISSIVE_7 as isize,
    Permissive8 = ffi::FOV_PERMISSIVE_8 as isize,
    Restrictive = ffi::FOV_RESTRICTIVE as isize,
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
    use std;
    use std::num::FromPrimitive;
    use std::old_io::fs::PathExtensions;
    use std::time::Duration;
    use ffi;
    use libc::c_char;
    use ::{c_bool,
           Key, EventFlags,
           ANY,
           KEY, KEY_RELEASE, KEY_PRESS,
           MOUSE, MOUSE_RELEASE, MOUSE_PRESS, MOUSE_MOVE};

    pub fn set_fps(fps: i32) {
        assert!(fps > 0);
        unsafe {
            ffi::TCOD_sys_set_fps(fps)
        }
    }

    pub fn get_fps() -> i32 {
        let mut result;
        unsafe {
            result = ffi::TCOD_sys_get_fps();
        }
        assert!(result >= 0);
        return result
    }

    pub fn get_last_frame_length() -> f32 {
        unsafe {
            ffi::TCOD_sys_get_last_frame_length() as f32
        }
    }

    pub fn sleep(time: Duration) {
        unsafe {
            ffi::TCOD_sys_sleep_milli(time.num_milliseconds() as u32);
        }
    }

    pub fn get_elapsed_time() -> Duration {
        let ms: u32 = unsafe {
            ffi::TCOD_sys_elapsed_milli()
        };
        return Duration::milliseconds(ms as i64)
    }

    pub fn save_screenshot(path: &std::old_path::Path) {
        assert!(path.exists());
        let c_path = std::ffi::CString::from_slice(path.as_vec());
        unsafe {
            ffi::TCOD_sys_save_screenshot(c_path.as_ptr());
        }
    }

    pub fn save_screenshot_auto() {
        unsafe {
            ffi::TCOD_sys_save_screenshot(std::ptr::null());
        }
    }

    pub fn force_fullscreen_resolution(width: i32, height: i32) {
        assert!(width > 0 && height > 0);
        unsafe {
            ffi::TCOD_sys_force_fullscreen_resolution(width, height);
        }
    }

    pub fn get_current_resolution() -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            ffi::TCOD_sys_get_current_resolution(&mut width, &mut height);
        }
        (width, height)
    }

    pub fn get_fullscreen_offset() -> (i32, i32) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        unsafe {
            ffi::TCOD_sys_get_fullscreen_offsets(&mut x, &mut y);
        }
        (x, y)
    }

    pub fn get_char_size() -> (i32, i32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            ffi::TCOD_sys_get_char_size(&mut width, &mut height);
        }
        (width, height)
    }

    pub fn set_clipboard(value: &str) {
        let c_str = std::ffi::CString::from_slice(value.as_bytes());
        unsafe {
            ffi::TCOD_sys_clipboard_set(c_str.as_ptr());
        }
    }

    pub fn get_clipboard() -> String {
        unsafe {
            let c_ptr = ffi::TCOD_sys_clipboard_get();
            let c_str = std::ffi::c_str_to_bytes(&c_ptr);
            std::str::from_utf8(c_str).unwrap().to_string()
        }
    }

    pub fn check_for_event(event_mask: EventFlags) -> (EventFlags, Event) {
        let mut c_key_state = ffi::TCOD_key_t {
            vk: 0,
            c: ' ' as c_char,
            pressed: false as c_bool,
            lalt: false as c_bool,
            lctrl: false as c_bool,
            ralt: false as c_bool,
            rctrl: false as c_bool,
            shift: false as c_bool
        };

        let mut c_mouse_state = ffi::TCOD_mouse_t {
            x: 0,
            y: 0,
            dx: 0,
            dy: 0,
            cx: 0,
            cy: 0,
            dcx: 0,
            dcy: 0,
            lbutton: false as c_bool,
            rbutton: false as c_bool,
            mbutton: false as c_bool,
            lbutton_pressed: false as c_bool,
            rbutton_pressed: false as c_bool,
            mbutton_pressed: false as c_bool,
            wheel_up: false as c_bool,
            wheel_down: false as c_bool
        };

        let event = unsafe {
            ffi::TCOD_sys_check_for_event(
                event_mask.bits() as i32,
                if event_mask.intersects(KEY_PRESS|KEY_RELEASE|KEY|ANY) {
                    &mut c_key_state
                } else {
                    std::ptr::null_mut()
                },
                if event_mask.intersects(
                    MOUSE_MOVE|MOUSE_PRESS|MOUSE_RELEASE|MOUSE|ANY) {
                    &mut c_mouse_state
                } else {
                    std::ptr::null_mut()
                })
        };

        let ret_flag = match event {
            ffi::TCOD_EVENT_KEY_PRESS => KEY_PRESS,
            ffi::TCOD_EVENT_KEY_RELEASE => KEY_RELEASE,
            ffi::TCOD_EVENT_KEY => KEY,
            ffi::TCOD_EVENT_MOUSE => MOUSE,
            ffi::TCOD_EVENT_MOUSE_MOVE => MOUSE_MOVE,
            ffi::TCOD_EVENT_MOUSE_PRESS => MOUSE_PRESS,
            ffi::TCOD_EVENT_MOUSE_RELEASE => MOUSE_RELEASE,
            _ => ANY
        };

        let ret_event = if ret_flag == ANY {
            Event::None
        } else if ret_flag.intersects(KEY_PRESS|KEY_RELEASE|KEY) {
            Event::Key(::KeyState {
                key: if c_key_state.vk == ffi::TCODK_CHAR {
                    Key::Printable(c_key_state.c as u8 as char)
                } else {
                    Key::Special(FromPrimitive::from_u32(c_key_state.vk)
                                 .unwrap())
                },
                pressed: c_key_state.pressed != 0,
                left_alt: c_key_state.lalt != 0,
                left_ctrl: c_key_state.lctrl != 0,
                right_alt: c_key_state.ralt != 0,
                right_ctrl: c_key_state.rctrl != 0,
                shift: c_key_state.shift != 0
            })
        } else if ret_flag.intersects(MOUSE_MOVE|MOUSE_PRESS|MOUSE_RELEASE|MOUSE) {
            Event::Mouse(::MouseState {
                x: c_mouse_state.x as isize,
                y: c_mouse_state.y as isize,
                dx: c_mouse_state.dx as isize,
                dy: c_mouse_state.dy as isize,
                cx: c_mouse_state.cx as isize,
                cy: c_mouse_state.cy as isize,
                dcx: c_mouse_state.dcx as isize,
                dcy: c_mouse_state.dcy as isize,
                lbutton: c_mouse_state.lbutton != 0,
                rbutton: c_mouse_state.rbutton != 0,
                mbutton: c_mouse_state.mbutton != 0,
                lbutton_pressed: c_mouse_state.lbutton_pressed != 0,
                rbutton_pressed: c_mouse_state.rbutton_pressed != 0,
                mbutton_pressed: c_mouse_state.mbutton_pressed != 0,
                wheel_up: c_mouse_state.wheel_up != 0,
                wheel_down: c_mouse_state.wheel_down != 0
            })
        } else {
            Event::None
        };

        (ret_flag, ret_event)
    }

    #[derive(Copy, Debug)]
    pub enum Event {
        Key(::KeyState),
        Mouse(::MouseState),
        None
    }
}

pub mod mouse {
    use ffi;
    use ::c_bool;

    pub fn show_cursor(visible: bool) {
        unsafe {
            ffi::TCOD_mouse_show_cursor(visible as c_bool);
        }
    }

    pub fn is_cursor_visible() -> bool {
        unsafe {
            ffi::TCOD_mouse_is_cursor_visible() != 0
        }
    }

    pub fn move_cursor(x: i32, y: i32) {
        unsafe {
            ffi::TCOD_mouse_move(x, y);
        }
    }
}

bitflags! {
    flags KeyPressFlags: c_uint {
        const KEY_PRESSED = ffi::TCOD_KEY_PRESSED,
        const KEY_RELEASED = ffi::TCOD_KEY_RELEASED,
    }
}

bitflags! {
    flags EventFlags: c_uint {
        const KEY_PRESS = ffi::TCOD_EVENT_KEY_PRESS,
        const KEY_RELEASE = ffi::TCOD_EVENT_KEY_RELEASE,
        const KEY = ffi::TCOD_EVENT_KEY,
        const MOUSE_MOVE = ffi::TCOD_EVENT_MOUSE_MOVE,
        const MOUSE_PRESS = ffi::TCOD_EVENT_MOUSE_PRESS,
        const MOUSE_RELEASE = ffi::TCOD_EVENT_MOUSE_RELEASE,
        const MOUSE = ffi::TCOD_EVENT_MOUSE,
        const ANY = ffi::TCOD_EVENT_ANY,
    }
}
