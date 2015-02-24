#![feature(libc, std_misc, old_path, core, old_io)]

extern crate libc;
extern crate "tcod-sys" as ffi;
#[macro_use] extern crate bitflags;

use libc::{c_int, c_uint, c_float, uint8_t, c_void};

pub use Console::Root as RootConsole;
pub use colors::Color;

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
            let c_title = CString::new(title.as_bytes()).unwrap();
            ffi::TCOD_console_init_root(width, height,
                                        c_title.as_ptr(),
                                        fullscreen as c_bool,
                                        ffi::TCOD_RENDERER_SDL);
        }
        Console::Root
    }

    pub fn is_fullscreen() -> bool {
        unsafe {
            ffi::TCOD_console_is_fullscreen() != 0
        }
    }

    pub fn set_fullscreen(fullscreen: bool) {
        unsafe {
            ffi::TCOD_console_set_fullscreen(fullscreen as u8);
        }
    }

    pub fn disable_keyboard_repeat() {
        unsafe {
            ffi::TCOD_console_disable_keyboard_repeat()
        }
    }

    pub fn is_active() -> bool {
        unsafe {
            ffi::TCOD_console_is_active() != 0
        }
    }

    pub fn get_alignment(&self) -> TextAlignment {
        let alignment = unsafe {
            ffi::TCOD_console_get_alignment(self.con())
        };
        match alignment {
            ffi::TCOD_LEFT => TextAlignment::Left,
            ffi::TCOD_RIGHT => TextAlignment::Right,
            ffi::TCOD_CENTER => TextAlignment::Center,
            _ => unreachable!(),
        }
    }

    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        unsafe {
            ffi::TCOD_console_set_alignment(self.con(), alignment as u32);
        }
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
            ffi::TCOD_console_set_key_color(self.con(), color.to_color_t());
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
            ffi::TCOD_console_set_default_background(self.con(), color.to_color_t());
        }
    }

    pub fn set_default_foreground(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_foreground(self.con(), color.to_color_t());
        }
    }

    pub fn console_set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con(), color.to_color_t());
        }
    }

    pub fn get_char_background(&self, x: i32, y: i32) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_console_get_char_background(self.con(), x, y))
        }
    }

    pub fn get_char_foreground(&self, x: i32, y: i32) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_console_get_char_foreground(self.con(), x, y))
        }
    }

    pub fn get_background_flag(&self) -> BackgroundFlag {
        let flag = unsafe {
            ffi::TCOD_console_get_background_flag(self.con())
        };
        match flag {
            ffi::TCOD_BKGND_NONE => BackgroundFlag::None,
            ffi::TCOD_BKGND_SET => BackgroundFlag::Set,
            ffi::TCOD_BKGND_MULTIPLY => BackgroundFlag::Multiply,
            ffi::TCOD_BKGND_LIGHTEN => BackgroundFlag::Lighten,
            ffi::TCOD_BKGND_DARKEN => BackgroundFlag::Darken,
            ffi::TCOD_BKGND_SCREEN => BackgroundFlag::Screen,
            ffi::TCOD_BKGND_COLOR_DODGE => BackgroundFlag::ColorDodge,
            ffi::TCOD_BKGND_COLOR_BURN => BackgroundFlag::ColorBurn,
            ffi::TCOD_BKGND_ADD => BackgroundFlag::Add,
            ffi::TCOD_BKGND_ADDA => BackgroundFlag::AddA,
            ffi::TCOD_BKGND_BURN => BackgroundFlag::Burn,
            ffi::TCOD_BKGND_OVERLAY => BackgroundFlag::Overlay,
            ffi::TCOD_BKGND_ALPH => BackgroundFlag::Alph,
            ffi::TCOD_BKGND_DEFAULT => BackgroundFlag::Default,
            _ => unreachable!(),
        }
    }

    pub fn set_background_flag(&mut self, background_flag: BackgroundFlag) {
        unsafe {
            ffi::TCOD_console_set_background_flag(self.con(),
                                                  background_flag as u32);
        }
    }

    pub fn get_char(&self, x: i32, y: i32) -> char {
        let ffi_char = unsafe {
            ffi::TCOD_console_get_char(self.con(), x, y)
        };
        assert!(ffi_char >= 0 && ffi_char < 256);
        ffi_char as u8 as char
    }

    pub fn set_char(&mut self, x: i32, y: i32, c: char) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char(self.con(), x, y, c as i32)
        }
    }

    pub fn set_char_background(&mut self, x: i32, y: i32,
                               color: Color,
                               background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_background(self.con(),
                                                  x, y,
                                                  color.to_color_t(),
                                                  background_flag as u32)
        }
    }

    pub fn set_char_foreground(&mut self, x: i32, y: i32, color: Color) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_foreground(self.con(),
                                                  x, y,
                                                  color.to_color_t());
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
                                          foreground.to_color_t(),
                                          background.to_color_t());
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            ffi::TCOD_console_clear(self.con());
        }
    }

    pub fn print(&mut self, x: i32, y: i32, text: &str) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            let c_text = CString::new(text.as_bytes()).unwrap();
            ffi::TCOD_console_print(self.con(), x, y, c_text.as_ptr());
        }
    }

    pub fn print_ex(&mut self,
                    x: i32, y: i32,
                    background_flag: BackgroundFlag,
                    alignment: TextAlignment,
                    text: &str) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            let c_text = CString::new(text.as_bytes()).unwrap();
            ffi::TCOD_console_print_ex(self.con(),
                                        x, y,
                                        background_flag as u32,
                                        alignment as u32,
                                        c_text.as_ptr());
        }
    }

    pub fn get_fade() -> u8 {
        unsafe {
            ffi::TCOD_console_get_fade()
        }
    }

    pub fn get_fading_color() -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_console_get_fading_color())
        }
    }

    pub fn set_fade(fade: u8, fading_color: Color) {
        unsafe {
            ffi::TCOD_console_set_fade(fade, fading_color.to_color_t());
        }
    }

    pub fn set_custom_font(font_path: ::std::old_path::Path, flags: FontFlags,
                           nb_char_horizontal: i32,
                           nb_char_vertical: i32) {
        unsafe {
            let path = CString::new(font_path.as_vec()).unwrap();
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
            let c_title = CString::new(title.as_bytes()).unwrap();
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

    pub fn walk(&mut self) -> AStarPathIterator {
        AStarPathIterator{tcod_path: self.tcod_path.ptr, recalculate: false}
    }

    pub fn walk_recalculate(&mut self) -> AStarPathIterator {
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

    pub fn walk(&mut self) -> DijkstraPathIterator {
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

pub struct AStarPathIterator {
    tcod_path: ffi::TCOD_path_t,
    recalculate: bool,
}

impl Iterator for AStarPathIterator {
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

pub struct DijkstraPathIterator {
    tcod_path: ffi::TCOD_path_t,
}

impl Iterator for DijkstraPathIterator {
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
    #![allow(non_upper_case_globals)]
    use super::ffi;

    #[repr(C)]
    #[derive(Copy, Debug, PartialEq)]
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
        let c_path = std::ffi::CString::new(path.as_vec()).unwrap();
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
        let c_str = std::ffi::CString::new(value.as_bytes()).unwrap();
        unsafe {
            ffi::TCOD_sys_clipboard_set(c_str.as_ptr());
        }
    }

    pub fn get_clipboard() -> String {
        unsafe {
            let c_ptr = ffi::TCOD_sys_clipboard_get();
            let c_str = std::ffi::CStr::from_ptr(c_ptr).to_bytes();
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

pub mod chars {
    use ffi;

    #[allow(non_camel_case_types)]
    pub type TCOD_char_t = ::libc::c_uint;

    pub const TCOD_CHAR_HLINE: TCOD_char_t = ffi::TCOD_CHAR_HLINE;
    pub const TCOD_CHAR_VLINE: TCOD_char_t = ffi::TCOD_CHAR_VLINE;
    pub const TCOD_CHAR_NE: TCOD_char_t = ffi::TCOD_CHAR_NE;
    pub const TCOD_CHAR_NW: TCOD_char_t = ffi::TCOD_CHAR_NW;
    pub const TCOD_CHAR_SE: TCOD_char_t = ffi::TCOD_CHAR_SE;
    pub const TCOD_CHAR_SW: TCOD_char_t = ffi::TCOD_CHAR_SW;
    pub const TCOD_CHAR_TEEW: TCOD_char_t = ffi::TCOD_CHAR_TEEW;
    pub const TCOD_CHAR_TEEE: TCOD_char_t = ffi::TCOD_CHAR_TEEE;
    pub const TCOD_CHAR_TEEN: TCOD_char_t = ffi::TCOD_CHAR_TEEN;
    pub const TCOD_CHAR_TEES: TCOD_char_t = ffi::TCOD_CHAR_TEES;
    pub const TCOD_CHAR_CROSS: TCOD_char_t = ffi::TCOD_CHAR_CROSS;
    pub const TCOD_CHAR_DHLINE: TCOD_char_t = ffi::TCOD_CHAR_DHLINE;
    pub const TCOD_CHAR_DVLINE: TCOD_char_t = ffi::TCOD_CHAR_DVLINE;
    pub const TCOD_CHAR_DNE: TCOD_char_t = ffi::TCOD_CHAR_DNE;
    pub const TCOD_CHAR_DNW: TCOD_char_t = ffi::TCOD_CHAR_DNW;
    pub const TCOD_CHAR_DSE: TCOD_char_t = ffi::TCOD_CHAR_DSE;
    pub const TCOD_CHAR_DSW: TCOD_char_t = ffi::TCOD_CHAR_DSW;
    pub const TCOD_CHAR_DTEEW: TCOD_char_t = ffi::TCOD_CHAR_DTEEW;
    pub const TCOD_CHAR_DTEEE: TCOD_char_t = ffi::TCOD_CHAR_DTEEE;
    pub const TCOD_CHAR_DTEEN: TCOD_char_t = ffi::TCOD_CHAR_DTEEN;
    pub const TCOD_CHAR_DTEES: TCOD_char_t = ffi::TCOD_CHAR_DTEES;
    pub const TCOD_CHAR_DCROSS: TCOD_char_t = ffi::TCOD_CHAR_DCROSS;
    pub const TCOD_CHAR_BLOCK1: TCOD_char_t = ffi::TCOD_CHAR_BLOCK1;
    pub const TCOD_CHAR_BLOCK2: TCOD_char_t = ffi::TCOD_CHAR_BLOCK2;
    pub const TCOD_CHAR_BLOCK3: TCOD_char_t = ffi::TCOD_CHAR_BLOCK3;
    pub const TCOD_CHAR_ARROW_N: TCOD_char_t = ffi::TCOD_CHAR_ARROW_N;
    pub const TCOD_CHAR_ARROW_S: TCOD_char_t = ffi::TCOD_CHAR_ARROW_S;
    pub const TCOD_CHAR_ARROW_E: TCOD_char_t = ffi::TCOD_CHAR_ARROW_E;
    pub const TCOD_CHAR_ARROW_W: TCOD_char_t = ffi::TCOD_CHAR_ARROW_W;
    pub const TCOD_CHAR_ARROW2_N: TCOD_char_t = ffi::TCOD_CHAR_ARROW2_N;
    pub const TCOD_CHAR_ARROW2_S: TCOD_char_t = ffi::TCOD_CHAR_ARROW2_S;
    pub const TCOD_CHAR_ARROW2_E: TCOD_char_t = ffi::TCOD_CHAR_ARROW2_E;
    pub const TCOD_CHAR_ARROW2_W: TCOD_char_t = ffi::TCOD_CHAR_ARROW2_W;
    pub const TCOD_CHAR_DARROW_H: TCOD_char_t = ffi::TCOD_CHAR_DARROW_H;
    pub const TCOD_CHAR_DARROW_V: TCOD_char_t = ffi::TCOD_CHAR_DARROW_V;
    pub const TCOD_CHAR_CHECKBOX_UNSET: TCOD_char_t = ffi::TCOD_CHAR_CHECKBOX_UNSET;
    pub const TCOD_CHAR_CHECKBOX_SET: TCOD_char_t = ffi::TCOD_CHAR_CHECKBOX_SET;
    pub const TCOD_CHAR_RADIO_UNSET: TCOD_char_t = ffi::TCOD_CHAR_RADIO_UNSET;
    pub const TCOD_CHAR_RADIO_SET: TCOD_char_t = ffi::TCOD_CHAR_RADIO_SET;
    pub const TCOD_CHAR_SUBP_NW: TCOD_char_t = ffi::TCOD_CHAR_SUBP_NW;
    pub const TCOD_CHAR_SUBP_NE: TCOD_char_t = ffi::TCOD_CHAR_SUBP_NE;
    pub const TCOD_CHAR_SUBP_N: TCOD_char_t = ffi::TCOD_CHAR_SUBP_N;
    pub const TCOD_CHAR_SUBP_SE: TCOD_char_t = ffi::TCOD_CHAR_SUBP_SE;
    pub const TCOD_CHAR_SUBP_DIAG: TCOD_char_t = ffi::TCOD_CHAR_SUBP_DIAG;
    pub const TCOD_CHAR_SUBP_E: TCOD_char_t = ffi::TCOD_CHAR_SUBP_E;
    pub const TCOD_CHAR_SUBP_SW: TCOD_char_t = ffi::TCOD_CHAR_SUBP_SW;
    pub const TCOD_CHAR_SMILIE: TCOD_char_t = ffi::TCOD_CHAR_SMILIE;
    pub const TCOD_CHAR_SMILIE_INV: TCOD_char_t = ffi::TCOD_CHAR_SMILIE_INV;
    pub const TCOD_CHAR_HEART: TCOD_char_t = ffi::TCOD_CHAR_HEART;
    pub const TCOD_CHAR_DIAMOND: TCOD_char_t = ffi::TCOD_CHAR_DIAMOND;
    pub const TCOD_CHAR_CLUB: TCOD_char_t = ffi::TCOD_CHAR_CLUB;
    pub const TCOD_CHAR_SPADE: TCOD_char_t = ffi::TCOD_CHAR_SPADE;
    pub const TCOD_CHAR_BULLET: TCOD_char_t = ffi::TCOD_CHAR_BULLET;
    pub const TCOD_CHAR_BULLET_INV: TCOD_char_t = ffi::TCOD_CHAR_BULLET_INV;
    pub const TCOD_CHAR_MALE: TCOD_char_t = ffi::TCOD_CHAR_MALE;
    pub const TCOD_CHAR_FEMALE: TCOD_char_t = ffi::TCOD_CHAR_FEMALE;
    pub const TCOD_CHAR_NOTE: TCOD_char_t = ffi::TCOD_CHAR_NOTE;
    pub const TCOD_CHAR_NOTE_DOUBLE: TCOD_char_t = ffi::TCOD_CHAR_NOTE_DOUBLE;
    pub const TCOD_CHAR_LIGHT: TCOD_char_t = ffi::TCOD_CHAR_LIGHT;
    pub const TCOD_CHAR_EXCLAM_DOUBLE: TCOD_char_t = ffi::TCOD_CHAR_EXCLAM_DOUBLE;
    pub const TCOD_CHAR_PILCROW: TCOD_char_t = ffi::TCOD_CHAR_PILCROW;
    pub const TCOD_CHAR_SECTION: TCOD_char_t = ffi::TCOD_CHAR_SECTION;
    pub const TCOD_CHAR_POUND: TCOD_char_t = ffi::TCOD_CHAR_POUND;
    pub const TCOD_CHAR_MULTIPLICATION: TCOD_char_t = ffi::TCOD_CHAR_MULTIPLICATION;
    pub const TCOD_CHAR_FUNCTION: TCOD_char_t = ffi::TCOD_CHAR_FUNCTION;
    pub const TCOD_CHAR_RESERVED: TCOD_char_t = ffi::TCOD_CHAR_RESERVED;
    pub const TCOD_CHAR_HALF: TCOD_char_t = ffi::TCOD_CHAR_HALF;
    pub const TCOD_CHAR_ONE_QUARTER: TCOD_char_t = ffi::TCOD_CHAR_ONE_QUARTER;
    pub const TCOD_CHAR_COPYRIGHT: TCOD_char_t = ffi::TCOD_CHAR_COPYRIGHT;
    pub const TCOD_CHAR_CENT: TCOD_char_t = ffi::TCOD_CHAR_CENT;
    pub const TCOD_CHAR_YEN: TCOD_char_t = ffi::TCOD_CHAR_YEN;
    pub const TCOD_CHAR_CURRENCY: TCOD_char_t = ffi::TCOD_CHAR_CURRENCY;
    pub const TCOD_CHAR_THREE_QUARTERS: TCOD_char_t = ffi::TCOD_CHAR_THREE_QUARTERS;
    pub const TCOD_CHAR_DIVISION: TCOD_char_t = ffi::TCOD_CHAR_DIVISION;
    pub const TCOD_CHAR_GRADE: TCOD_char_t = ffi::TCOD_CHAR_GRADE;
    pub const TCOD_CHAR_UMLAUT: TCOD_char_t = ffi::TCOD_CHAR_UMLAUT;
    pub const TCOD_CHAR_POW1: TCOD_char_t = ffi::TCOD_CHAR_POW1;
    pub const TCOD_CHAR_POW3: TCOD_char_t = ffi::TCOD_CHAR_POW3;
    pub const TCOD_CHAR_POW2: TCOD_char_t = ffi::TCOD_CHAR_POW2;
    pub const TCOD_CHAR_BULLET_SQUARE: TCOD_char_t = ffi::TCOD_CHAR_BULLET_SQUARE;
}
