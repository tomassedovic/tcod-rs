use std::ptr;
use std::str;
use std::ffi::{CStr, CString};
use std::path::Path;
use std::sync::Mutex;

use bindings::ffi;
use bindings::{AsNative, c_char};
use random::Rng;

static mut NAMEGEN_FREE: bool = true;
lazy_static! {
    static ref NAMEGEN_MUTEX: Mutex<()> = Mutex::new(());
}

pub struct Namegen {
    rng: Vec<Rng>,
}

impl Drop for Namegen {
    fn drop(&mut self) {
        unsafe {
            let _lock = NAMEGEN_MUTEX.lock()
                .ok()
                .expect("Namegen mutex could not be locked");
            if self.rng.is_empty() {
                ffi::TCOD_namegen_destroy();
            }
            NAMEGEN_FREE = true;
        }
    }
}

impl Namegen {
    pub fn new() -> Option<Namegen> {
        unsafe {
            match NAMEGEN_FREE {
                true => {
                    let _lock = NAMEGEN_MUTEX.lock()
                        .ok()
                        .expect("Namegen mutex could not be locked");
                    NAMEGEN_FREE = false;
                    Some(Namegen { rng: Vec::new() })
                },
                false => None
            }
        }
    }

    pub fn parse<T>(&mut self, path: T) where T: AsRef<Path> {
        self.parse_with_rng(path, &Rng::get_instance())
    }

    pub fn parse_with_rng<T>(&mut self, path: T, rng: &Rng) where T: AsRef<Path> {
        self.rng.push(rng.save());

        let path_string = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            ffi::TCOD_namegen_parse(path_string.as_ptr(), *self.rng.last().unwrap().as_native());
        }
    }

    pub fn generate<T>(&self, name: T) -> Option<String> where T: AsRef<str> {
        unsafe {
            let name_string = CString::new(name.as_ref()).unwrap();
            let borrowed = ffi::TCOD_namegen_generate(name_string.as_ptr() as *mut _, 0);
            cstr_to_owned(borrowed)
        }
    }

    pub fn generate_custom<T, U>(&self, name: T, rule: U) -> Option<String> where T: AsRef<str>, U: AsRef<str> {
        unsafe {
            let name_string = CString::new(name.as_ref()).unwrap();
            let rule_string = CString::new(rule.as_ref()).unwrap();

            let borrowed = ffi::TCOD_namegen_generate_custom(name_string.as_ptr() as *mut _,
                                                             rule_string.as_ptr() as *mut _, 0);
            cstr_to_owned(borrowed)
        }
    }

    pub fn get_sets(&self) -> Vec<String> {
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
