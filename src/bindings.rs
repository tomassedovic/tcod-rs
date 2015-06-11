extern crate std;
extern crate libc;
extern crate tcod_sys as ffi;

pub use self::std::ffi::CString;
pub use self::libc::{c_char, c_int, c_float, c_uint, c_void, uint8_t};

use super::input::KeyCode;

#[allow(non_camel_case_types)]
pub type c_bool = uint8_t;

pub trait AsNative<T> {
    unsafe fn as_native(&self) -> &T;
}

pub trait FromNative<T> {
    unsafe fn from_native(input: T) -> Self;
}

impl<'a, T, U: AsNative<T> + ?Sized> AsNative<T> for &'a U {
    unsafe fn as_native(&self) -> &T {
        (**self).as_native()
    }
}

impl <T, U: AsNative<T> + ?Sized> AsNative<T> for Box<U> {
    unsafe fn as_native(&self) -> &T {
        (**self).as_native()
    }
}

pub fn keycode_from_u32(input: u32) -> Option<KeyCode> {
    match input {
        0 => Some(KeyCode::NoKey),
        1 => Some(KeyCode::Escape),
        2 => Some(KeyCode::Backspace),
        3 => Some(KeyCode::Tab),
        4 => Some(KeyCode::Enter),
        5 => Some(KeyCode::Shift),
        6 => Some(KeyCode::Control),
        7 => Some(KeyCode::Alt),
        8 => Some(KeyCode::Pause),
        9 => Some(KeyCode::Capslock),
        10 => Some(KeyCode::Pageup),
        11 => Some(KeyCode::Pagedown),
        12 => Some(KeyCode::End),
        13 => Some(KeyCode::Home),
        14 => Some(KeyCode::Up),
        15 => Some(KeyCode::Left),
        16 => Some(KeyCode::Right),
        17 => Some(KeyCode::Down),
        18 => Some(KeyCode::PrintScreen),
        19 => Some(KeyCode::Insert),
        20 => Some(KeyCode::Delete),
        21 => Some(KeyCode::LeftWin),
        22 => Some(KeyCode::RightWin),
        23 => Some(KeyCode::Apps),
        24 => Some(KeyCode::Number0),
        25 => Some(KeyCode::Number1),
        26 => Some(KeyCode::Number2),
        27 => Some(KeyCode::Number3),
        28 => Some(KeyCode::Number4),
        29 => Some(KeyCode::Number5),
        30 => Some(KeyCode::Number6),
        31 => Some(KeyCode::Number7),
        32 => Some(KeyCode::Number8),
        33 => Some(KeyCode::Number9),
        34 => Some(KeyCode::NumPad0),
        35 => Some(KeyCode::NumPad1),
        36 => Some(KeyCode::NumPad2),
        37 => Some(KeyCode::NumPad3),
        38 => Some(KeyCode::NumPad4),
        39 => Some(KeyCode::NumPad5),
        40 => Some(KeyCode::NumPad6),
        41 => Some(KeyCode::NumPad7),
        42 => Some(KeyCode::NumPad8),
        43 => Some(KeyCode::NumPad9),
        44 => Some(KeyCode::NumPadAdd),
        45 => Some(KeyCode::NumPadSubtract),
        46 => Some(KeyCode::NumPadDivide),
        47 => Some(KeyCode::NumPadMultiply),
        48 => Some(KeyCode::NumPadDecimal),
        49 => Some(KeyCode::NumPadEnter),
        50 => Some(KeyCode::F1),
        51 => Some(KeyCode::F2),
        52 => Some(KeyCode::F3),
        53 => Some(KeyCode::F4),
        54 => Some(KeyCode::F5),
        55 => Some(KeyCode::F6),
        56 => Some(KeyCode::F7),
        57 => Some(KeyCode::F8),
        58 => Some(KeyCode::F9),
        59 => Some(KeyCode::F10),
        60 => Some(KeyCode::F11),
        61 => Some(KeyCode::F12),
        62 => Some(KeyCode::NUMLOCK),
        63 => Some(KeyCode::SCROLLLOCK),
        64 => Some(KeyCode::Spacebar),
        65 => Some(KeyCode::Char),
        _ => None,
    }
}
