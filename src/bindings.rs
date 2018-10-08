pub extern crate tcod_sys as ffi;

pub use std::ffi::{CStr, CString};
pub use std::os::raw::{c_char, c_int, c_float, c_uint, c_void, c_uchar};

use std::mem::transmute;
use super::input::KeyCode;

#[allow(non_camel_case_types)]
pub type c_bool = c_uchar;

pub trait AsNative<T> {
    unsafe fn as_native(&self) -> &T;
    unsafe fn as_native_mut(&mut self) -> &mut T;
}

pub trait FromNative<T> {
    unsafe fn from_native(input: T) -> Self;
}

impl<'a, T, U: AsNative<T> + ?Sized> AsNative<T> for &'a U {
    unsafe fn as_native(&self) -> &T {
        (**self).as_native()
    }
    
    unsafe fn as_native_mut(&mut self) -> &mut T {
        unimplemented!();
    }
}

impl <T, U: AsNative<T> + ?Sized> AsNative<T> for Box<U> {
    unsafe fn as_native(&self) -> &T {
        (**self).as_native()
    }
    
    unsafe fn as_native_mut(&mut self) -> &mut T {
        (**self).as_native_mut()
    }
}

pub fn keycode_from_native(input: self::ffi::TCOD_keycode_t) -> Option<KeyCode> {
    match input as u32 {
        x @ 0 ... 66 => Some(unsafe { transmute(x) }),
        _ => None
    }
}

#[test]
fn test_keycodes() {
    assert!(keycode_from_native(self::ffi::TCOD_keycode_t::TCODK_NONE).unwrap() == KeyCode::NoKey);
    assert!(keycode_from_native(self::ffi::TCOD_keycode_t::TCODK_6).unwrap() == KeyCode::Number6);
    assert!(keycode_from_native(self::ffi::TCOD_keycode_t::TCODK_TEXT).unwrap() == KeyCode::Text);
}
