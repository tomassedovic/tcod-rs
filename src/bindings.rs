extern crate std;
extern crate libc;
extern crate tcod_sys as ffi;

pub use self::std::ffi::CString;
pub use self::libc::{c_char, c_int, c_float, c_uint, c_void, uint8_t};

#[allow(non_camel_case_types)]
pub type c_bool = uint8_t;

