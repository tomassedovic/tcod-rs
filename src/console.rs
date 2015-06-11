//! The console emulator handles the rendering of the game screen and the keyboard input
//!
//! It provides the necessary traits and types for working with the different console types,
//! including the [Console](./trait.Console.html) trait and the
//! [Root](./struct.Root.html) and [Offscreen](./struct.Offscreen.html) console types.
//! It's worth mentioning that only one `Root` console may exist at any given time, and it has to
//! be initialized at the start of the program.
//!
//! # Examples
//!
//! Initializing the `Root` console and creating an `Offscreen` console:
//!
//! ```rust
//! use tcod::console::{Root, Offscreen};
//!
//! let mut root = Root::initializer().init();
//! let (width, height) = (80, 30);
//! let mut offscreen = Offscreen::new(width, height);
//! ```
//!
//! A typical `tcod-rs` program's basic structure would look something like this:
//!
//! ```rust
//! use tcod::console::Root;
//!
//! fn main() {
//!     let mut root = Root::initializer().init(); // Replace with custom initialization code
//!
//!     while !root.window_closed() {
//!         // Handling user input
//!         // Updating the gamestate
//!         // Rendering the results
//!     }
//! }
//! ```
//!
//! For detailed examples on the user input handling and rendering see the
//! [Root](./struct.Root.html) struct's documentation.
//!
//!
//! ## Additional Information
//!
//! The `Root` and `Console` types are also reexported in the root module (`tcod`) under the names
//! `RootConsole` and `OffscreenConsole`, making the following code sample equivalent to the
//! previous one:
//!
//! ```rust
//! use tcod::{RootConsole, OffscreenConsole};
//!
//! let mut root = RootConsole::initializer().init();
//! let (width, height) = (80, 30);
//! let mut offscreen = OffscreenConsole::new(width, height);
//! ```
//! This applies to all the examples in the rest of the modules documentation.


extern crate std;

use std::ascii::AsciiExt;
use std::marker::PhantomData;
use std::path::Path;

use bindings::ffi;
use bindings::{AsNative, FromNative, c_bool, c_char, CString, keycode_from_u32};

use colors::Color;
use input::{Key, KeyPressFlags, KeyState};

/// A type representing secondary consoles
///
/// `Offscreen` consoles allow you draw on secondary consoles as you would on `Root` consoles, and then
/// `blit` their contents onto other consoles (including `Root`). There are some limitations
/// compared to `Root` consoles, however:
///
/// * Functions manipulating the main window or handling user input are limited to the `Root`
/// console
/// * `Offscreen` consoles may not be `flushed` to the screen directly
///
/// # Examples
///
/// Creating an `Offscreen` console
///
/// ```rust
/// use tcod::console::Offscreen;
///
/// let width = 80;
/// let height = 20;
/// let offscreen = Offscreen::new(width, height);
/// ```
///
/// Blitting an `Offscreen` console to the `Root` console:
///
/// ```rust
/// use tcod::console as console;
/// use tcod::console::{Root, Offscreen};
///
/// fn main() {
///     let mut root = Root::initializer().init();
///
///     let mut direct = Offscreen::new(20, 20);
///     console::blit(&direct, (0, 0), (20, 20), &mut root, (0, 0), 1.0, 1.0);
/// }
///
/// ```
///
/// See the documentation for [blit](./fn.blit.html) for a detailed description of the function parameters
/// and a more in-depth example.
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
    /// Creates a new `Offscreen` console instance
    pub fn new(width: i32, height: i32) -> Offscreen {
        assert!(width > 0 && height > 0);
        unsafe {
            Offscreen { con: ffi::TCOD_console_new(width, height) }
        }
    }

}

/// The console representing the main window of the application
///
/// This is the only console type capable of handling user input and flushing its contents onto the screen.
/// There may only be one Root console at any given time, and it should be initialized at the start of the program.
///
/// # Examples
///
/// ## Handling user input
/// `tcod-rs` provides two ways of handling user input: blocking or non-blocking. The following
/// exaple will show the blocking method
///
/// ```rust
/// use tcod::console::Root;
/// use tcod::input::Key::Special;
/// use tcod::input::KeyCode::{Up, Down, Left, Right};
///
/// fn main() {
///     let mut root = Root::initializer().init();
///
///     let keypress = root.wait_for_keypress(true);
///     match keypress.key {
///         Special(Up) => {}, // Handle arrow key up
///         Special(Down) => {}, // Arrow key down
///         Special(Left) => {},
///         Special(Right) => {},
///         _ => {}
///     }
/// }
/// ```
///
/// For a detailed description of possible values of `keypress.key` values see the
/// [Key](../input/enum.Key.html) and [KeyCode](../input/enum.KeyCode.html) enums.
///
/// ## Rendering
/// `libtcod` provides a wide variety of functions for changing how the console output looks like,
/// including: changing the text and background colors, text alignment, etc. It also has
/// several functions that are used to output text on consoles. For a complete list of both,
/// see the [Console](./trait.Console.html) trait's documentation. The basic structure of the
/// rendering code:
///
/// ```rust
/// use tcod::console::{Console, Root};
///
/// fn main() {
///     let mut root = Root::initializer().init();
///
///     root.clear();
///     // Output style manipulation
///     // Calling the output functions
///     root.flush();
/// }
/// ```

struct RootId {
    id: ffi::TCOD_console_t
}

unsafe impl Sync for RootId {}

static ROOT_ID: RootId = RootId { id: 0 as ffi::TCOD_console_t };

pub struct Root {
    // This is here to prevent the explicit creation of Root consoles.
    _blocker: PhantomData<Root>
}

impl Root {
    /// Returns an instance of a RootInitializer object, which can be used to
    /// customize the initialization of the Root console. Note that only
    /// `RootInitializer::init` will return the actual `Root` console instance.
    /// For a full list of initialization options, see the
    /// [RootInitializer](./struct.RootInitializer.html) documentation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tcod::console::Root;
    ///
    /// let mut root = Root::initializer()
    ///     .size(80, 20)
    ///     .title("Example")
    ///     .fullscreen(true)
    ///     .init();
    /// ```
    ///
     pub fn initializer<'a>() -> RootInitializer<'a> {
        RootInitializer::new()
    }

    /// Returns with true when the `Root` console is in fullscreen mode.
    pub fn is_fullscreen(&self) -> bool {
        unsafe {
            ffi::TCOD_console_is_fullscreen() != 0
        }
    }

    /// Toggles between windowed and fullscreen mode.
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        unsafe {
            ffi::TCOD_console_set_fullscreen(fullscreen as u8);
        }
    }

    /// This function changes the keyboard repeat times. The initial delay determines the
    /// number of milliseconds between the keypress and the time the keyboard repeat begins.
    /// The interval sets the time between the keyboard repeat events.
    /// With an initial delay of 0, the keyboard repeat feature is completely disabled.
    pub fn set_keyboard_repeat(&mut self, initial_delay: i32, interval: i32) {
        unsafe {
            ffi::TCOD_console_set_keyboard_repeat(initial_delay, interval);
        }
    }

    /// Disables the keyboard repeat feature. Equivalent to `set_keyboard_repeat` with an
    /// initial delay of 0.
    pub fn disable_keyboard_repeat(&mut self) {
        unsafe {
            ffi::TCOD_console_disable_keyboard_repeat()
        }
    }

    /// Returns true if the `Root` console is currently active.
    pub fn is_active(&self) -> bool {
        unsafe {
            ffi::TCOD_console_is_active() != 0
        }
    }

    /// Returns the current fade amount (previously set by `set_fade`).
    pub fn get_fade(&self) -> u8 {
        unsafe {
            ffi::TCOD_console_get_fade()
        }
    }

    /// Returns the current fade color (previously set by `set_fade`).
    pub fn get_fading_color(&self) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_console_get_fading_color())
        }
    }

    /// This function defines the fading parameters, allowing to easily fade the game screen to/from a color.
    /// Once they are defined, the fading parameters are valid for ever.
    /// You don't have to call setFade for each rendered frame (unless you change the fading parameters).
    pub fn set_fade(&mut self, fade: u8, fading_color: Color) {
        unsafe {
            ffi::TCOD_console_set_fade(fade, *fading_color.as_native());
        }
    }

    /// This function will wait for a keypress event from the user, returning the [KeyState](../input/struct.KeyState.html)
    /// that represents the event. If `flush` is true, all pending keypresses are flushed from the
    /// keyboard buffer. If false, it returns the first element from it.
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

    /// This function checks if the user pressed a key. It returns the
    /// [KeyState](../input/struct.KeyState.html) representing the
    /// event if they have, or `None` if they have not.
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

    /// Returns with true if the `Root` console has been closed.
    pub fn window_closed(&self) -> bool {
        unsafe {
            ffi::TCOD_console_is_window_closed() != 0
        }
    }

    /// Flushes the contents of the `Root` console onto the screen.
    pub fn flush(&mut self) {
        unsafe {
            ffi::TCOD_console_flush();
        }
    }

    /// Sets the main window's title to the string specified in the argument.
    pub fn set_window_title<T>(&mut self, title: T) where T: AsRef<str> {
        unsafe {
            let c_title = CString::new(title.as_ref().as_bytes()).unwrap();
            ffi::TCOD_console_set_window_title(c_title.as_ptr());
        }
    }

    fn set_custom_font(font_path: &Path,
                       font_layout: FontLayout,
                       font_type: FontType,
                       nb_char_horizontal: i32,
                       nb_char_vertical: i32) {
        unsafe {
            let filename = font_path.to_str().unwrap();
            let path = CString::new(filename).unwrap();
            ffi::TCOD_console_set_custom_font(
                path.as_ptr(), (font_layout as i32) | (font_type as i32),
                nb_char_horizontal, nb_char_vertical);
        }
    }

}

struct FontDimensions(i32, i32);

/// Helper struct for the `Root` console initialization
///
/// This is the type that should be used to initialize the `Root` console (either directly or
/// indirectly, by calling `Root::initializer`). It uses method chaining to provide an easy-to-use
/// interface. It exposes the following configuration options for the `Root` console:
///
/// * `size`: this determines the size of the console window in characters
/// * `title`: the main window's title
/// * `fullscreen`: determines if the main window will start in fullscreen mode
/// * `font`: selects a bitmap font and sets its layout. See [FontLayout](./enum.FontLayout.html)
/// for the possible layouts. The `path` argument can be a
/// [`str`](http://doc.rust-lang.org/std/primitive.str.html),
/// [`Path`](http://doc.rust-lang.org/std/path/struct.Path.html),
/// [`String`](http://doc.rust-lang.org/std/string/struct.String.html) or anything else that
/// implements [`AsRef<Path>`](http://doc.rust-lang.org/std/convert/trait.AsRef.html).
/// * `font_type`: only use this if you want to use a greyscale font. See
/// [FontType](./enum.FontType.html) for the possible values.
/// * `font_dimensions`: the dimensions for the given bitmap font. This is automatically
/// deduced from the font layout, only use this if you really need it (providing wrong values will
/// ruin the font display).
/// * `renderer`: sets the console renderer. See the [Renderer](./enum.Renderer.html) enum for the
/// valid options.
///
/// The initializer provides sane defaults even there are no options explicitly specified, but it
/// is recommended to at least set the size and the window title.
///
/// # Examples
///
/// Initializing the `Root` console using `Root::initializer` instead of explicitly creating a
/// `RootInitializer` instance:
///
/// ```rust
/// use tcod::console::{Root, FontLayout, Renderer};
///
/// fn main() {
///     let mut root = Root::initializer()
///         .size(80, 20)
///         .title("Example")
///         .fullscreen(true)
///         .font("terminal.png", FontLayout::AsciiInCol)
///         .renderer(Renderer::GLSL)
///         .init();
/// }
/// ```
pub struct RootInitializer<'a> {
    width: i32,
    height: i32,
    title: Box<AsRef<str> + 'a>,
    is_fullscreen: bool,
    font_path: Box<AsRef<Path> + 'a>,
    font_layout: FontLayout,
    font_type: FontType,
    font_dimension: FontDimensions,
    console_renderer: Renderer
}

impl<'a> RootInitializer<'a> {
    pub fn new() -> RootInitializer<'a> {
        RootInitializer {
            width: 80,
            height: 25,
            title: Box::new("Main Window"),
            is_fullscreen: false,
            font_path: Box::new("terminal.png"),
            font_layout: FontLayout::AsciiInCol,
            font_type: FontType::Default,
            font_dimension: FontDimensions(0, 0),
            console_renderer: Renderer::SDL
        }
    }

    pub fn size(&mut self, width: i32, height: i32) -> &mut RootInitializer<'a> {
        self.width = width;
        self.height = height;
        self
    }

    pub fn title<T>(&mut self, title: T) -> &mut RootInitializer<'a> where T: AsRef<str> + 'a {
        assert!(title.as_ref().is_ascii());
        self.title = Box::new(title);
        self
    }

    pub fn fullscreen(&mut self, is_fullscreen: bool) -> &mut RootInitializer<'a> {
        self.is_fullscreen = is_fullscreen;
        self
    }

    pub fn font<P>(&mut self, path: P, font_layout: FontLayout) -> &mut RootInitializer<'a> where P: AsRef<Path> + 'a {
        self.font_path = Box::new(path);
        self.font_layout = font_layout;
        self
    }

    pub fn font_type(&mut self, font_type: FontType) -> &mut RootInitializer<'a> {
        self.font_type = font_type;
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
        assert!(self.width > 0 && self.height > 0);

        match self.font_dimension {
            FontDimensions(horizontal, vertical) => {
                Root::set_custom_font(self.font_path.as_ref(),
                                      self.font_layout, self.font_type,
                                      horizontal, vertical)
            }
        }

        unsafe {
            let c_title = CString::new(self.title.as_ref().as_bytes()).unwrap();
            ffi::TCOD_console_init_root(self.width, self.height,
                                        c_title.as_ptr(),
                                        self.is_fullscreen as c_bool,
                                        self.console_renderer as u32);
        }
        Root { _blocker: PhantomData }
    }
}

/// Defines the common functionality between `Root` and `Offscreen` consoles
///
/// # Examples
/// Printing text with explicit alignment:
///
/// ```rust
/// use tcod::console::{Console, Root, BackgroundFlag, TextAlignment};
///
/// let mut root = Root::initializer().size(80, 50).init();
///
/// root.print_ex(1, 1, BackgroundFlag::None, TextAlignment::Left,
///               "Text aligned to left.");
///
/// root.print_ex(78, 1, BackgroundFlag::None, TextAlignment::Right,
///               "Text aligned to right.");
///
/// root.print_ex(40, 15, BackgroundFlag::None, TextAlignment::Center,
///               "And this bit of text is centered.");
///
/// root.print_ex(40, 19, BackgroundFlag::None, TextAlignment::Center,
///               "Press any key to quit.");
/// ```
pub trait Console : AsNative<ffi::TCOD_console_t> {
    /// Returns the default text alignment for the `Console` instance. For all the possible
    /// text alignment options, see the documentation for
    /// [TextAlignment](./enum.TextAlignment.html).
    fn get_alignment(&self) -> TextAlignment {
        let alignment = unsafe {
            ffi::TCOD_console_get_alignment(*self.as_native())
        };
        match alignment {
            ffi::TCOD_LEFT => TextAlignment::Left,
            ffi::TCOD_RIGHT => TextAlignment::Right,
            ffi::TCOD_CENTER => TextAlignment::Center,
            _ => unreachable!(),
        }
    }

    /// Sets the default text alignment for the console. For all the possible
    /// text alignment options, see the documentation for
    /// [TextAlignment](./enum.TextAlignment.html).
    fn set_alignment(&mut self, alignment: TextAlignment) {
        unsafe {
            ffi::TCOD_console_set_alignment(*self.as_native(), alignment as u32);
        }
    }

    /// Sets a key color that will be ignored when [blitting](./fn.blit.html) the contents
    /// of this console onto an other (essentially a transparent background color).
    fn set_key_color(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_key_color(*self.as_native(), *color.as_native());
        }
    }

    /// Returns the width of the console in characters.
    fn width(&self) -> i32 {
        unsafe {
            ffi::TCOD_console_get_width(*self.as_native())
        }
    }

    /// Returns the height of the console in characters.
    fn height(&self) -> i32 {
        unsafe {
            ffi::TCOD_console_get_height(*self.as_native())
        }
    }

    /// Sets the console's default background color. This is used in several other methods,
    /// like: `clear`, `put_char`, etc.
    fn set_default_background(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_background(*self.as_native(), *color.as_native());
        }
    }

    /// Sets the console's default foreground color. This is used in several printing functions.
    fn set_default_foreground(&mut self, color: Color) {
        unsafe {
            ffi::TCOD_console_set_default_foreground(*self.as_native(), *color.as_native());
        }
    }

    /// Returns the background color of the cell at the specified coordinates.
    fn get_char_background(&self, x: i32, y: i32) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_console_get_char_background(*self.as_native(), x, y))
        }
    }

    /// Returns the foreground color of the cell at the specified coordinates.
    fn get_char_foreground(&self, x: i32, y: i32) -> Color {
        unsafe {
            FromNative::from_native(
                ffi::TCOD_console_get_char_foreground(*self.as_native(), x, y))
        }
    }

    /// Returns the console's current background flag. For a detailed explanation
    /// of the possible values, see [BackgroundFlag](./enum.BackgroundFlag.html).
    fn get_background_flag(&self) -> BackgroundFlag {
        let flag = unsafe {
            ffi::TCOD_console_get_background_flag(*self.as_native())
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

    /// Sets the console's current background flag. For a detailed explanation
    /// of the possible values, see [BackgroundFlag](./enum.BackgroundFlag.html).
    fn set_background_flag(&mut self, background_flag: BackgroundFlag) {
        unsafe {
            ffi::TCOD_console_set_background_flag(*self.as_native(),
                                                  background_flag as u32);
        }
    }

    /// Returns the ASCII value of the cell located at `x, y`
    fn get_char(&self, x: i32, y: i32) -> char {
        let ffi_char = unsafe {
            ffi::TCOD_console_get_char(*self.as_native(), x, y)
        };
        assert!(ffi_char >= 0 && ffi_char < 256);
        ffi_char as u8 as char
    }

    /// Modifies the ASCII value of the cell located at `x, y`.
    fn set_char(&mut self, x: i32, y: i32, c: char) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char(*self.as_native(), x, y, c as i32)
        }
    }

    /// Changes the background color of the specified cell
    fn set_char_background(&mut self, x: i32, y: i32,
                           color: Color,
                           background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_background(*self.as_native(),
                                                  x, y,
                                                  *color.as_native(),
                                                  background_flag as u32)
        }
    }

    /// Changes the foreground color of the specified cell
    fn set_char_foreground(&mut self, x: i32, y: i32, color: Color) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_set_char_foreground(*self.as_native(),
                                                  x, y,
                                                  *color.as_native());
        }
    }

    /// This function modifies every property of the given cell:
    ///
    /// 1. Updates its background color according to the console's default and `background_flag`,
    /// see [BackgroundFlag](./enum.BackgroundFlag.html).
    /// 2. Updates its foreground color based on the default color set in the console
    /// 3. Sets its ASCII value to `glyph`
    fn put_char(&mut self,
                x: i32, y: i32, glyph: char,
                background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char(*self.as_native(),
                                       x, y, glyph as i32,
                                       background_flag as u32);
        }
    }

    /// Updates every propert of the given cell using explicit colors for the
    /// background and foreground.
    fn put_char_ex(&mut self,
                   x: i32, y: i32, glyph: char,
                   foreground: Color, background: Color) {
        assert!(x >= 0 && y >= 0);
        unsafe {
            ffi::TCOD_console_put_char_ex(*self.as_native(),
                                          x, y, glyph as i32,
                                          *foreground.as_native(),
                                          *background.as_native());
        }
    }

    /// Clears the console with its default background color
    fn clear(&mut self) {
        unsafe {
            ffi::TCOD_console_clear(*self.as_native());
        }
    }

    /// Prints the text at the specified location. The position of the `x` and `y`
    /// coordinates depend on the [TextAlignment](./enum.TextAlignment.html) set in the console:
    ///
    /// * `TextAlignment::Left`: leftmost character of the string
    /// * `TextAlignment::Center`: center character of the sting
    /// * `TextAlignment::Right`: rightmost character of the string
    fn print<T>(&mut self, x: i32, y: i32, text: T) where Self: Sized, T: AsRef<str> {
        assert!(x >= 0 && y >= 0);
        let text = text.as_ref();
        if text.is_ascii() {
            let c_text = CString::new(text.as_bytes()).unwrap();
            unsafe {
                ffi::TCOD_console_print(*self.as_native(), x, y, c_text.as_ptr());
            }
        } else {
            let c_text = text.chars().collect::<Vec<_>>();
            unsafe {
                ffi::TCOD_console_print_utf(*self.as_native(), x, y, c_text.as_ptr() as *const i32);
            }
        }
    }

    /// Prints the text at the specified location in a rectangular area with
    /// the dimensions: (width; height). If the text is longer than the width the
    /// newlines will be inserted.
    fn print_rect<T>(&mut self,
                  x: i32, y: i32,
                  width: i32, height: i32,
                  text: T) where Self: Sized, T: AsRef<str> {
        assert!(x >= 0 && y >= 0);
        let text = text.as_ref();
        if text.is_ascii() {
            let c_text = CString::new(text.as_bytes()).unwrap();
            unsafe {
                ffi::TCOD_console_print_rect(*self.as_native(), x, y, width, height, c_text.as_ptr());
            }
        } else {
            let c_text = text.chars().collect::<Vec<_>>();
            unsafe {
                ffi::TCOD_console_print_rect_utf(*self.as_native(), x, y, width, height, c_text.as_ptr() as *const i32);
            }
        }
    }

    /// Prints the text at the specified location with an explicit
    /// [BackgroundFlag](./enum.BackgroundFlag.html) and
    /// [TextAlignment](./enum.TextAlignment.html).
    fn print_ex<T>(&mut self,
                x: i32, y: i32,
                background_flag: BackgroundFlag,
                alignment: TextAlignment,
                text: T) where Self: Sized, T: AsRef<str> {
        assert!(x >= 0 && y >= 0);
        let text = text.as_ref();
        if text.is_ascii() {
            let c_text = CString::new(text.as_bytes()).unwrap();
            unsafe {
                ffi::TCOD_console_print_ex(*self.as_native(), x, y,
                                           background_flag as u32,
                                           alignment as u32,
                                           c_text.as_ptr());
            }
        } else {
            let c_text = text.chars().collect::<Vec<_>>();
            unsafe {
                ffi::TCOD_console_print_ex_utf(*self.as_native(), x, y,
                                               background_flag as u32,
                                               alignment as u32,
                                               c_text.as_ptr() as *const i32);
            }
        }
    }

    /// Combines the functions of `print_ex` and `print_rect`
    fn print_rect_ex<T>(&mut self,
                        x: i32, y: i32,
                        width: i32, height: i32,
                        background_flag: BackgroundFlag,
                        alignment: TextAlignment,
                        text: T) where Self: Sized, T: AsRef<str> {
        assert!(x >= 0 && y >= 0);
        let text = text.as_ref();
        if text.is_ascii() {
            let c_text = CString::new(text.as_bytes()).unwrap();
            unsafe {
                ffi::TCOD_console_print_rect_ex(*self.as_native(), x, y, width, height,
                                                background_flag as u32, alignment as u32,
                                                c_text.as_ptr());
            }
        } else {
            let c_text = text.chars().collect::<Vec<_>>();
            unsafe {
                ffi::TCOD_console_print_rect_ex_utf(*self.as_native(), x, y, width, height,
                                                    background_flag as u32, alignment as u32,
                                                    c_text.as_ptr() as *const i32);
            }
        }
    }

    /// Fill a rectangle with the default background colour.
    ///
    /// If `clear` is true, set each cell's character to space (ASCII 32).
    fn rect(&mut self,
            x: i32, y: i32,
            width: i32, height: i32,
            clear: bool,
            background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0 && width >= 0 && height >= 0);
        assert!(x + width < self.width() && y + height < self.height());
        unsafe {
            ffi::TCOD_console_rect(*self.as_native(), x, y, width, height, clear as c_bool, background_flag as u32);
        }
    }

    /// Draw a horizontal line.
    ///
    /// Uses `tcod::chars::HLINE` (ASCII 196) as the line character and
    /// console's default background and foreground colours.
    fn horizontal_line(&mut self, x: i32, y: i32, length: i32, background_flag: BackgroundFlag) {
        assert!(x >= 0 && y >= 0 && y < self.height());
        assert!(length >= 1 && length + x < self.width());
        unsafe {
            ffi::TCOD_console_hline(*self.as_native(), x, y, length, background_flag as u32);
        }
    }

    /// Draw a vertical line.
    ///
    /// Uses `tcod::chars::VLINE` (ASCII 179) as the line character and
    /// console's default background and foreground colours.
    fn vertical_line(&mut self, x: i32, y: i32, length: i32, background_flag: BackgroundFlag) {
        assert!(x >= 0, y >= 0 && x < self.width());
        assert!(length >= 1 && length + y < self.height());
        unsafe {
            ffi::TCOD_console_vline(*self.as_native(), x, y, length, background_flag as u32);
        }
    }

    /// Draw a window frame with an optional title.
    ///
    /// Draws a rectangle (using the rect method) using the suplied background
    /// flag, then draws a rectangle with the console's default foreground
    /// colour.
    ///
    /// If the `title` is specified, it will be printed on top of the rectangle
    /// using inverted colours.
    fn print_frame<T>(&mut self, x: i32, y: i32, width: i32, height: i32,
                     clear: bool, background_flag: BackgroundFlag, title: Option<T>) where Self: Sized, T: AsRef<str> {
        assert!(x >= 0 && y >= 0 && width >= 0 && height >= 0);
        assert!(x + width < self.width() && y + height < self.height());
        let c_title: *const c_char = match title {
            Some(s) => {
                assert!(s.as_ref().is_ascii());
                CString::new(s.as_ref()).unwrap().as_ptr()
            },
            None => std::ptr::null(),
        };
        unsafe {
            ffi::TCOD_console_print_frame(*self.as_native(), x, y, width, height,
                                          clear as c_bool, background_flag as u32, c_title);
        }
    }
}

/// Blits the contents of one console onto an other
///
/// It takes a region from a given console (with an arbitrary location, width and height) and superimposes
/// it on the destination console (at the given location).
/// Note that when blitting, the source console's key color (set by `set_key_color`) will
/// be ignored, making it possible to blit non-rectangular regions.
///
/// # Arguments
///
/// * `source_console`: the type implementing the [Console](./trait.Console.html) trait we want to
/// take the blitted region from
/// * `source_x`, `source_y`: the coordinates of the blitted region's top left corner on the source
/// console
/// * `source_width`, `source_height`: the width and height of the blitted region. With a value of
/// 0, the width and height of the source console will be used.
/// * `destination_console`: the type implementing the [Console](./trait.Console.html) trait we want
/// to blit to
/// * `destination_x`, `destination_y`: the coordinated of the blitted region's top left corner on
/// the destination console
/// * `foreground_alpha`, `background_alpha`: the foreground and background opacity
///
/// # Examples
///
/// Using `blit` with concrete types and `Console` trait objects:
///
/// ```rust
/// use tcod::console as console;
/// use tcod::console::{Console, Root, Offscreen};
///
/// fn main() {
///     let mut root = Root::initializer().init();
///
///     let mut direct = Offscreen::new(20, 20);
///     let mut boxed_direct = Box::new(Offscreen::new(20, 20));
///     let mut trait_object: &Console = &Offscreen::new(20, 20);
///     let mut boxed_trait: Box<Console> = Box::new(Offscreen::new(20, 20));
///
///     console::blit(&direct, (0, 0), (20, 20), &mut root, (0, 0), 1.0, 1.0);
///     console::blit(&boxed_direct, (0, 0), (20, 20), &mut root, (20, 0), 1.0, 1.0);
///     console::blit(&trait_object, (0, 0), (20, 20), &mut root, (0, 20), 1.0, 1.0);
///     console::blit(&boxed_trait, (0, 0), (20, 20), &mut root, (20, 20), 1.0, 1.0);
/// }
///
/// ```
pub fn blit<T, U>(source_console: &T,
                  (source_x, source_y): (i32, i32),
                  (source_width, source_height): (i32, i32),
                  destination_console: &mut U,
                  (destination_x, destination_y): (i32, i32),
                  foreground_alpha: f32, background_alpha: f32)
    where T: Console,
          U: Console {
    assert!(source_x >= 0 && source_y >= 0 &&
            source_width >= 0 && source_height >= 0 && // If width or height is 0, the source width/height is used.
            destination_x >= 0 && destination_y >= 0);

    unsafe {
        ffi::TCOD_console_blit(*source_console.as_native(),
                               source_x, source_y, source_width, source_height,
                               *destination_console.as_native(),
                               destination_x, destination_y,
                               foreground_alpha, background_alpha);
    }
}

impl<'a, T: Console + ?Sized> Console for &'a T {}

impl<T: Console + ?Sized> Console for Box<T> {}

impl AsNative<ffi::TCOD_console_t> for Root {
    unsafe fn as_native(&self) -> &ffi::TCOD_console_t {
        &ROOT_ID.id
    }
}

impl AsNative<ffi::TCOD_console_t> for Offscreen {
    unsafe fn as_native(&self) -> &ffi::TCOD_console_t {
        &self.con
    }
}

impl Console for Root {}
impl Console for Offscreen {}

/// Represents the text alignment in console instances.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum TextAlignment {
    Left = ffi::TCOD_LEFT as isize,
    Right = ffi::TCOD_RIGHT as isize,
    Center = ffi::TCOD_CENTER as isize,
}

/// This flag determines how a cell's existing background color will be modified by a new one
///
/// See [libtcod's documentation](http://doryen.eptalys.net/data/libtcod/doc/1.5.2/html2/console_bkgnd_flag_t.html)
/// for a detailed description of the different values.
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

/// All the possible renderers used by the `Root` console
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Renderer {
    GLSL = ffi::TCOD_RENDERER_GLSL as isize,
    OpenGL = ffi::TCOD_RENDERER_OPENGL as isize,
    SDL = ffi::TCOD_RENDERER_SDL as isize,
}

/// All the possible font layouts that can be used for custom bitmap fonts
#[repr(C)]
#[derive(Copy, Clone)]
pub enum FontLayout {
    AsciiInCol = ffi::TCOD_FONT_LAYOUT_ASCII_INCOL as isize,
    AsciiInRow = ffi::TCOD_FONT_LAYOUT_ASCII_INROW as isize,
    Tcod = ffi::TCOD_FONT_LAYOUT_TCOD as isize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FontType {
    Default = 0,
    Greyscale = ffi::TCOD_FONT_TYPE_GREYSCALE as isize,
}



#[cfg(test)]
mod test {
    use std::path::Path;
    use super::Root;
    use super::FontLayout::AsciiInCol;

    #[test]
    fn test_custom_font_as_static_str() {
        Root::initializer().font("terminal.png", AsciiInCol);
    }

    #[test]
    fn test_custom_font_as_path() {
        Root::initializer().font(Path::new("terminal.png"), AsciiInCol);

    }

    #[test]
    fn test_custom_font_as_string() {
        Root::initializer().font("terminal.png".to_owned(), AsciiInCol);
    }

    #[test]
    fn test_custom_font_as_str() {
        let string = "terminal.png".to_owned();
        let s: &str = &string;
        Root::initializer().font(s, AsciiInCol);
    }
}
