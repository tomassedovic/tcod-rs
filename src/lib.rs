#![feature(std_misc, core, path_ext)]
#![allow(dead_code)]

#[macro_use] extern crate bitflags;

pub use console::*;
pub use input::{EventFlags, Key, KeyCode, KeyPressFlags, KeyState, MouseState};
pub use map::{Map, FovAlgorithm};
pub use pathfinding::*;
pub use system::*;

pub use Console::Root as RootConsole;
pub use colors::Color;
pub use input::{ANY,
                KEY, KEY_RELEASE, KEY_PRESS,
                MOUSE, MOUSE_RELEASE, MOUSE_PRESS, MOUSE_MOVE};


pub mod chars;
pub mod colors;
pub mod input;
pub mod system;

mod bindings;
mod console;
mod map;
mod pathfinding;









