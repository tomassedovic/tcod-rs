pub extern crate tcod_sys as ffi;

extern crate libc;

pub use std::ffi::CString;
pub use self::libc::{c_char, c_int, c_float, c_uint, c_void, uint8_t};

use std::mem::transmute;
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
        x @ 0 ... 65 => Some(unsafe { transmute(x) }),
        _ => None
    }
}

#[test]
fn test_keycodes() {
    assert!(keycode_from_u32(0).unwrap() == KeyCode::NoKey);
    assert!(keycode_from_u32(30).unwrap() == KeyCode::Number6);
    assert!(keycode_from_u32(65).unwrap() == KeyCode::Char);
}
