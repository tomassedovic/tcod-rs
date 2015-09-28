use std::str;
use std::ptr;
use std::time::Duration;

use std::ffi::{CStr, CString};
use std::path::Path;
use bindings::ffi;

pub fn set_fps(fps: i32) {
    assert!(fps >= 0);
    unsafe {
        ffi::TCOD_sys_set_fps(fps)
    }
}

pub fn get_fps() -> i32 {
    let result = unsafe {
        ffi::TCOD_sys_get_fps()
    };
    assert!(result >= 0);
    result
}

pub fn get_last_frame_length() -> f32 {
    unsafe {
        ffi::TCOD_sys_get_last_frame_length()
    }
}

pub fn sleep(time: Duration) {
    let duration_ms = (time.as_secs() * 1000) as u32 + (time.subsec_nanos() / 1_000_000);
    unsafe {
        ffi::TCOD_sys_sleep_milli(duration_ms);
    }
}

pub fn get_elapsed_time() -> Duration {
    let ms: u32 = unsafe {
        ffi::TCOD_sys_elapsed_milli()
    };
    Duration::from_millis(ms as u64)
}

pub fn save_screenshot<P>(path: P) where P: AsRef<Path> {
    let filename = path.as_ref().to_str().expect("Invalid screenshot path");
    let c_path = CString::new(filename).unwrap();
    unsafe {
        ffi::TCOD_sys_save_screenshot(c_path.as_ptr());
    }
}

pub fn save_screenshot_auto() {
    unsafe {
        ffi::TCOD_sys_save_screenshot(ptr::null());
    }
}

pub fn force_fullscreen_resolution(width: i32, height: i32) {
    assert!(width > 0 && height > 0);
    unsafe {
        ffi::TCOD_sys_force_fullscreen_resolution(width, height);
    }
}

pub fn get_current_resolution() -> (i32, i32) {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    unsafe {
        ffi::TCOD_sys_get_current_resolution(&mut width, &mut height);
    }
    (width, height)
}

pub fn get_fullscreen_offset() -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    unsafe {
        ffi::TCOD_sys_get_fullscreen_offsets(&mut x, &mut y);
    }
    (x, y)
}

pub fn get_char_size() -> (i32, i32) {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    unsafe {
        ffi::TCOD_sys_get_char_size(&mut width, &mut height);
    }
    (width, height)
}

pub fn set_clipboard<T>(value: T) where T: AsRef<str> {
    let c_str = CString::new(value.as_ref().as_bytes()).unwrap();
    unsafe {
        ffi::TCOD_sys_clipboard_set(c_str.as_ptr());
    }
}

pub fn get_clipboard() -> String {
    unsafe {
        let c_ptr = ffi::TCOD_sys_clipboard_get();
        let c_str = CStr::from_ptr(c_ptr).to_bytes();
        str::from_utf8(c_str).unwrap().to_owned()
    }
}
