//! libtcod bindings for Rust
//!
//! ## Description
//! [libtcod a.k.a. "The Doryen Library"](http://roguecentral.org/doryen/libtcod/) is a
//! free, fast, portable and uncomplicated API for roguelike developpers providing lots of
//! useful functions, such as:
//!
//! * Text-based graphics API
//! * Colors
//! * Keyboard and mouse input
//! * Path finding
//! * Field of View
//! * Portability (works on Windows, Linux and OS X)
//!
//! For the full set of features see the [libtcod features page](http://roguecentral.org/doryen/libtcod/features/).
//!
//! All raw bindings are available via the `tcod-sys` crate, however the `tcod-rs` library aims to
//! provide safe, Rust-style wrappers for most of `libtcod`. These wrappers are not yet complete,
//! however.
//!
//! ### Features already implemented:
//!
//! * Colors
//! * Console
//! * Most of the System layer
//! * Field of View
//! * Map
//! * Pathfinding
//!
//! ### Features that are not planned to be implemented:
//! This are features that Rust already provides a good (and in most casese more idiomatic)
//! solution for:
//!
//! * Filesystem utilities
//! * Containers
//! * Pseudorandom generators
//! * Compression utilities
//!

#[macro_use] extern crate bitflags;

pub use bindings::{AsNative, FromNative};
pub use colors::Color;
pub use console::{Console, RootInitializer, BackgroundFlag, Renderer, FontLayout, FontType, TextAlignment};
pub use map::Map;

pub mod chars;
pub mod colors;
pub mod console;
pub mod input;
pub mod map;
pub mod pathfinding;
pub mod system;

mod bindings;
mod console_macros;

pub type RootConsole = console::Root;
pub type OffscreenConsole = console::Offscreen;
