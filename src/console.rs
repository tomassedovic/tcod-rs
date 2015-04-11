extern crate std;

pub use self::WindowType::Windowed as Windowed;
pub use self::WindowType::Fullscreen as Fullscreen;

use bindings::ffi;
use bindings::{c_bool, c_uint, CString, keycode_from_u32};

use colors::Color;
use input::{Key, KeyPressFlags, KeyState};

#[derive(Copy, Clone)]
pub enum WindowType {
    Windowed,
    Fullscreen
}

#[derive(Copy, Clone)]
pub struct FontDimensions(i32, i32);

#[derive(Copy, Clone)]
pub struct FontPath<'a>(&'a str);

pub struct Offscreen {
    con: ffi::TCOD_console_t,
}

impl Drop for Offscreen {
    fn drop(&mut self) {
        unsafe {
            ffi::TCOD_console_delete(self.con);
        }
    }
}

impl Offscreen {
    pub fn new(width: i32, height: i32) -> Offscreen {
        assert!(width > 0 && height > 0);
        unsafe {
            Offscreen { con: ffi::TCOD_console_new(width, height) }
        }
    }

}

pub struct Root;

impl Root {
    pub fn initializer<'a>() -> RootInitializer<'a> {
        RootInitializer::new()
    }
    
    fn init(width: i32, height: i32, title: &str, window_type: WindowType, 
            font_path: FontPath, flags: FontFlags, font_dimensions: FontDimensions,
            renderer: Renderer) -> Root {
        assert!(width > 0 && height > 0);
        
        match (font_path, font_dimensions) {
            (FontPath(path), FontDimensions(horizontal, vertical)) => {
                Root::set_custom_font(&std::path::Path::new(path), flags, horizontal, vertical)
            }
        }

        let fullscreen = match window_type {
            Fullscreen => true,
            Windowed => false
        };
        
        println!("{} {}", width, height);
        unsafe {
            let c_title = CString::new(title.as_bytes()).unwrap();
            ffi::TCOD_console_init_root(width, height,
                                        c_title.as_ptr(),
                                        fullscreen as c_bool,
                                        renderer as u32);
        }
        Root
    }
    
    
    pub fn is_fullscreen(&self) -> bool {
        unsafe {
            ffi::TCOD_console_is_fullscreen() != 0
        }
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        unsafe {
            ffi::TCOD_console_set_fullscreen(fullscreen as u8);
        }
    }

    pub fn disable_keyboard_repeat(&mut self) {
        unsafe {
            ffi::TCOD_console_disable_keyboard_repeat()
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe {
            ffi::TCOD_console_is_active() != 0
        }
    }


    pub fn get_fade(&self) -> u8 {
        unsafe {
            ffi::TCOD_console_get_fade()
        }
    }

    pub fn get_fading_color(&self) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_console_get_fading_color())
        }
    }

    pub fn set_fade(&mut self, fade: u8, fading_color: Color) {
        unsafe {
            ffi::TCOD_console_set_fade(fade, fading_color.to_color_t());
        }
    }

    pub fn wait_for_keypress(&mut self, flush: bool) -> KeyState {
        let tcod_key = unsafe {
            ffi::TCOD_console_wait_for_keypress(flush as c_bool)
        };
        let key = if tcod_key.vk == ffi::TCODK_CHAR {
            Key::Printable(tcod_key.c as u8 as char)
        } else {
            Key::Special(keycode_from_u32(tcod_key.vk).unwrap())
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

    pub fn check_for_keypress(&self, status: KeyPressFlags) -> Option<KeyState> {
        let tcod_key = unsafe {
            ffi::TCOD_console_check_for_keypress(status.bits() as i32)
        };
        if tcod_key.vk == ffi::TCODK_NONE {
            return None;
        }
        let key = if tcod_key.vk == ffi::TCODK_CHAR {
            Key::Printable(tcod_key.c as u8 as char)
        } else {
            Key::Special(keycode_from_u32(tcod_key.vk).unwrap())
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

    pub fn window_closed(&self) -> bool {
        unsafe {
            ffi::TCOD_console_is_window_closed() != 0
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            ffi::TCOD_console_flush();
        }
    }

    pub fn set_window_title(&mut self, title: &str) {
        unsafe {
            let c_title = CString::new(title.as_bytes()).unwrap();
            ffi::TCOD_console_set_window_title(c_title.as_ptr());
        }
    }
    
    fn set_custom_font(font_path: &std::path::Path, flags: FontFlags,
                       nb_char_horizontal: i32,
                       nb_char_vertical: i32) {
        unsafe {
            let filename = font_path.to_str().unwrap();
            let path = CString::new(filename).unwrap();
            ffi::TCOD_console_set_custom_font(
                path.as_ptr(), flags.bits() as i32, nb_char_horizontal,
                nb_char_vertical);
        }
    }

}

pub struct RootInitializer<'a> {
    width: i32,
    height: i32,
    title: &'a str,
    window_type: WindowType,
    font_path: FontPath<'a>,
    font_flags: FontFlags,
    font_dimension: FontDimensions,
    console_renderer: Renderer
}

impl<'a> RootInitializer<'a> {
    pub fn new() -> RootInitializer<'a> {
        RootInitializer {
            width: 80,
            height: 25,
            title: "Main Window",
            window_type: Windowed,
            font_path: FontPath("terminal.png"),
            font_flags: FONT_LAYOUT_ASCII_INCOL,
            font_dimension: FontDimensions(0, 0),
            console_renderer: Renderer::SDL
        }
    }

    pub fn size(&mut self, width: i32, height: i32) -> &mut RootInitializer<'a> {
        self.width = width;
        self.height = height;
        self
    }

    pub fn title(&mut self, title: &'a str) -> &mut RootInitializer<'a> {
        self.title = title;
        self
    }

    pub fn fullscreen(&mut self) -> &mut RootInitializer<'a> {
        self.window_type = Fullscreen;
        self
    }

    pub fn font(&mut self, path: &'a str, font_flags: FontFlags) -> &mut RootInitializer<'a> {
        self.font_path = FontPath(path);
        self.font_flags = font_flags;
        self
    }

    pub fn font_dimensions(&mut self, horizontal: i32, vertical: i32) -> &mut RootInitializer<'a> {
        self.font_dimension = FontDimensions(horizontal, vertical); 
        self
    }

    pub fn renderer(&mut self, renderer: Renderer) -> &mut RootInitializer<'a> {
        self.console_renderer = renderer;
        self
    }

    pub fn init(&self) -> Root {
        Root::init(self.width, self.height, self.title, self.window_type, 
                   self.font_path, self.font_flags, self.font_dimension, 
                   self.console_renderer)
    }
}

pub trait Console {
    unsafe fn con(&self) -> ffi::TCOD_console_t;

    fn get_alignment(&self) -> TextAlignment {
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
    
    fn set_alignment(&mut self, alignment: TextAlignment) {
        unsafe {
            ffi::TCOD_console_set_alignment(self.con(), alignment as u32);
        }
    }
     
    fn set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con(), color.to_color_t());
        }
    }

    fn width(&self) -> i32 {
        unsafe {
            ffi::TCOD_console_get_width(self.con())
        }
    }

    fn height(&self) -> i32 {
        unsafe {
            ffi::TCOD_console_get_height(self.con())
        }
    }

    fn set_default_background(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_background(self.con(), color.to_color_t());
        }
    }

    fn set_default_foreground(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_foreground(self.con(), color.to_color_t());
        }
    }

    fn console_set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(self.con(), color.to_color_t());
        }
    }

    fn get_char_background(&self, x: i32, y: i32) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_console_get_char_background(self.con(), x, y))
        }
    }

    fn get_char_foreground(&self, x: i32, y: i32) -> Color {
        unsafe {
            Color::from_tcod_color_t(
                ffi::TCOD_console_get_char_foreground(self.con(), x, y))
        }
    }

    fn get_background_flag(&self) -> BackgroundFlag {
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

    fn set_background_flag(&mut self, background_flag: BackgroundFlag) {
        unsafe {
            ffi::TCOD_console_set_background_flag(self.con(),
                                                  background_flag as u32);
        }
    }

    fn get_char(&self, x: i32, y: i32) -> char {
        let ffi_char = unsafe {
            ffi::TCOD_console_get_char(self.con(), x, y)
        };
        assert!(ffi_char >= 0 && ffi_char < 256);
        ffi_char as u8 as char
    }

    fn set_char(&mut self, x: i32, y: i32, c: char) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char(self.con(), x, y, c as i32)
        }
    }

    fn set_char_background(&mut self, x: i32, y: i32,
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

    fn set_char_foreground(&mut self, x: i32, y: i32, color: Color) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_foreground(self.con(),
                                                  x, y,
                                                  color.to_color_t());
        }
    }

    fn put_char(&mut self,
                x: i32, y: i32, glyph: char,
                background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char(self.con(),
                                       x, y, glyph as i32,
                                       background_flag as u32);
        }
    }

    fn put_char_ex(&mut self,
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

    fn clear(&mut self) {
        unsafe {
            ffi::TCOD_console_clear(self.con());
        }
    }

    fn print(&mut self, x: i32, y: i32, text: &str) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            let c_text = CString::new(text.as_bytes()).unwrap();
            ffi::TCOD_console_print(self.con(), x, y, c_text.as_ptr());
        }
    }

    fn print_ex(&mut self,
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
}

pub fn blit<T, U>(source_console: &T,
                  source_x: i32, source_y: i32,
                  source_width: i32, source_height: i32,
                  destination_console: &mut U,
                  destination_x: i32, destination_y: i32,
                  foreground_alpha: f32, background_alpha: f32)
    where T: Console,
          U: Console {
    unsafe {
        ffi::TCOD_console_blit(source_console.con(),
                               source_x, source_y, source_width, source_height,
                               destination_console.con(), 
                               destination_x, destination_y,
                               foreground_alpha, background_alpha);
    }
}

impl<'a> Console for &'a Console {
    unsafe fn con(&self) -> ffi::TCOD_console_t {
        (*self).con()
    }
}

impl Console for Box<Console> {
    unsafe fn con(&self) -> ffi::TCOD_console_t {
        (**self).con()
    }
}

impl Console for Root {
    unsafe fn con(&self) -> ffi::TCOD_console_t {
        0 as ffi::TCOD_console_t
    }
}

impl Console for Box<Root> {
    unsafe fn con(&self) -> ffi::TCOD_console_t {
        (**self).con()
    }
}

impl Console for Offscreen {
    unsafe fn con(&self) -> ffi::TCOD_console_t {
        self.con
    }
}

impl Console for Box<Offscreen> {
    unsafe fn con(&self) -> ffi::TCOD_console_t {
        (**self).con()
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub enum TextAlignment {
    Left = ffi::TCOD_LEFT as isize,
    Right = ffi::TCOD_RIGHT as isize,
    Center = ffi::TCOD_CENTER as isize,
}


#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
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
