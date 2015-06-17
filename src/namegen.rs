use std::ptr;
use std::str;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::path::Path;

use bindings::ffi;
use bindings::{AsNative, c_char};
use random::Rng;

pub struct Namegen {
    _blocker: PhantomData<Namegen>
}

#[inline]
fn cstr_to_owned(string: *mut c_char) -> Option<String> {
    if string == ptr::null::<c_char>() as *mut _ {
        return None;
    }

    unsafe {
        let string = CStr::from_ptr(string);
        str::from_utf8(string.to_bytes())
            .map(|x| x.to_owned())
            .ok()
    }
}

//TODO: Some kind of alternative API? This is not very Rusty, but global state is not very Rusty in
//general.
impl Namegen {
    pub fn parse<T>(path: T, rng: Rng) where T: AsRef<Path> {
        let path_string = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            ffi::TCOD_namegen_parse(path_string.as_ptr(), *rng.as_native());
        }
    }

    pub fn reset() {
        unsafe {
            ffi::TCOD_namegen_destroy();
        }
    }

    pub fn generate<T>(name: T) -> Option<String> where T: AsRef<str> {
        unsafe {
            let name_string = CString::new(name.as_ref()).unwrap();
            let borrowed = ffi::TCOD_namegen_generate(name_string.as_ptr() as *mut _, 0);
            cstr_to_owned(borrowed)
        }
    }

    pub fn generate_custom<T, U>(name: T, rule: U) -> Option<String> where T: AsRef<str>, U: AsRef<str> {
        unsafe {
            let name_string = CString::new(name.as_ref()).unwrap();
            let rule_string = CString::new(rule.as_ref()).unwrap();

            let borrowed = ffi::TCOD_namegen_generate_custom(name_string.as_ptr() as *mut _, rule_string.as_ptr() as *mut _, 0);
            cstr_to_owned(borrowed)
        }
    }

    pub fn get_sets() -> Vec<String> {
        unsafe {
            let list = ffi::TCOD_namegen_get_sets();
            let size = ffi::TCOD_list_size(list);
            let mut ret = Vec::with_capacity(size as usize);
            for i in 0..size {
                ret.push(cstr_to_owned(ffi::TCOD_list_get(list, i) as *mut c_char).unwrap());
            }
            ret
        }
    }
}


