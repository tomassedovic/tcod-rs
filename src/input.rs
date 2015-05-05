extern crate std;

use bindings::ffi;
use bindings::{c_bool, c_char, c_uint, keycode_from_u32};

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub enum KeyCode {
    NoKey,
    Escape,
    Backspace,
    Tab,
    Enter,
    Shift,
    Control,
    Alt,
    Pause,
    Capslock,
    Pageup,
    Pagedown,
    End,
    Home,
    Up,
    Left,
    Right,
    Down,
    PrintScreen,
    Insert,
    Delete,
    LeftWin,
    RightWin,
    Apps,
    // The numbers on the alphanum section of the keyboard
    Number0,
    Number1,
    Number2,
    Number3,
    Number4,
    Number5,
    Number6,
    Number7,
    Number8,
    Number9,
    // The numbers on the numeric keypad
    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumPadAdd,
    NumPadSubtract,
    NumPadDivide,
    NumPadMultiply,
    NumPadDecimal,
    NumPadEnter,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    NUMLOCK,
    SCROLLLOCK,
    Spacebar,
    Char,
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Key {
    Printable(char),
    Special(KeyCode),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct KeyState {
    pub key: Key,
    pub pressed: bool,
    pub left_alt: bool,
    pub left_ctrl: bool,
    pub right_alt: bool,
    pub right_ctrl: bool,
    pub shift: bool,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct MouseState {
    pub x: isize,
    pub y: isize,
    pub dx: isize,
    pub dy: isize,
    pub cx: isize,
    pub cy: isize,
    pub dcx: isize,
    pub dcy: isize,
    pub lbutton: bool,
    pub rbutton: bool,
    pub mbutton: bool,
    pub lbutton_pressed: bool,
    pub rbutton_pressed: bool,
    pub mbutton_pressed: bool,
    pub wheel_up: bool,
    pub wheel_down: bool,
}


pub fn show_cursor(visible: bool) {
    unsafe {
        ffi::TCOD_mouse_show_cursor(visible as c_bool);
    }
}

pub fn is_cursor_visible() -> bool {
    unsafe {
        ffi::TCOD_mouse_is_cursor_visible() != 0
    }
}

pub fn move_cursor(x: i32, y: i32) {
    unsafe {
        ffi::TCOD_mouse_move(x, y);
    }
}

bitflags! {
    flags KeyPressFlags: c_uint {
        const KEY_PRESSED = ffi::TCOD_KEY_PRESSED,
        const KEY_RELEASED = ffi::TCOD_KEY_RELEASED,
    }
}

bitflags! {
    flags EventFlags: c_uint {
        const KEY_PRESS = ffi::TCOD_EVENT_KEY_PRESS,
        const KEY_RELEASE = ffi::TCOD_EVENT_KEY_RELEASE,
        const KEY = ffi::TCOD_EVENT_KEY,
        const MOUSE_MOVE = ffi::TCOD_EVENT_MOUSE_MOVE,
        const MOUSE_PRESS = ffi::TCOD_EVENT_MOUSE_PRESS,
        const MOUSE_RELEASE = ffi::TCOD_EVENT_MOUSE_RELEASE,
        const MOUSE = ffi::TCOD_EVENT_MOUSE,
        const ANY = ffi::TCOD_EVENT_ANY,
    }
}

pub fn check_for_event(event_mask: EventFlags) -> Option<(EventFlags, Event)> {
    let mut c_key_state = ffi::TCOD_key_t {
        vk: 0,
        c: ' ' as c_char,
        pressed: false as c_bool,
        lalt: false as c_bool,
        lctrl: false as c_bool,
        ralt: false as c_bool,
        rctrl: false as c_bool,
        shift: false as c_bool
    };

    let mut c_mouse_state = ffi::TCOD_mouse_t {
        x: 0,
        y: 0,
        dx: 0,
        dy: 0,
        cx: 0,
        cy: 0,
        dcx: 0,
        dcy: 0,
        lbutton: false as c_bool,
        rbutton: false as c_bool,
        mbutton: false as c_bool,
        lbutton_pressed: false as c_bool,
        rbutton_pressed: false as c_bool,
        mbutton_pressed: false as c_bool,
        wheel_up: false as c_bool,
        wheel_down: false as c_bool
    };

    let event = unsafe {
        ffi::TCOD_sys_check_for_event(
            event_mask.bits() as i32,
            if event_mask.intersects(KEY_PRESS|KEY_RELEASE|KEY|ANY) {
                &mut c_key_state
            } else {
                std::ptr::null_mut()
            },
            if event_mask.intersects(
                MOUSE_MOVE|MOUSE_PRESS|MOUSE_RELEASE|MOUSE|ANY) {
                &mut c_mouse_state
            } else {
                std::ptr::null_mut()
            })
    };

    let ret_flag = match event {
        ffi::TCOD_EVENT_KEY_PRESS => KEY_PRESS,
        ffi::TCOD_EVENT_KEY_RELEASE => KEY_RELEASE,
        ffi::TCOD_EVENT_KEY => KEY,
        ffi::TCOD_EVENT_MOUSE => MOUSE,
        ffi::TCOD_EVENT_MOUSE_MOVE => MOUSE_MOVE,
        ffi::TCOD_EVENT_MOUSE_PRESS => MOUSE_PRESS,
        ffi::TCOD_EVENT_MOUSE_RELEASE => MOUSE_RELEASE,
        _ => ANY
    };

    if ret_flag == ANY {
        return None
    }

    let ret_event = if ret_flag.intersects(KEY_PRESS|KEY_RELEASE|KEY) {
        Some(Event::Key(KeyState {
            key: if c_key_state.vk == ffi::TCODK_CHAR {
                Key::Printable(c_key_state.c as u8 as char)
            } else {
                Key::Special(keycode_from_u32(c_key_state.vk)
                             .unwrap())
            },
            pressed: c_key_state.pressed != 0,
            left_alt: c_key_state.lalt != 0,
            left_ctrl: c_key_state.lctrl != 0,
            right_alt: c_key_state.ralt != 0,
            right_ctrl: c_key_state.rctrl != 0,
            shift: c_key_state.shift != 0
        }))
    } else if ret_flag.intersects(MOUSE_MOVE|MOUSE_PRESS|MOUSE_RELEASE|MOUSE) {
        Some(Event::Mouse(MouseState {
            x: c_mouse_state.x as isize,
            y: c_mouse_state.y as isize,
            dx: c_mouse_state.dx as isize,
            dy: c_mouse_state.dy as isize,
            cx: c_mouse_state.cx as isize,
            cy: c_mouse_state.cy as isize,
            dcx: c_mouse_state.dcx as isize,
            dcy: c_mouse_state.dcy as isize,
            lbutton: c_mouse_state.lbutton != 0,
            rbutton: c_mouse_state.rbutton != 0,
            mbutton: c_mouse_state.mbutton != 0,
            lbutton_pressed: c_mouse_state.lbutton_pressed != 0,
            rbutton_pressed: c_mouse_state.rbutton_pressed != 0,
            mbutton_pressed: c_mouse_state.mbutton_pressed != 0,
            wheel_up: c_mouse_state.wheel_up != 0,
            wheel_down: c_mouse_state.wheel_down != 0
        }))
    } else {
        None
    };

    if ret_event.is_some() {
        Some((ret_flag, ret_event.unwrap()))
    } else {
        None
    }
}

pub fn events() -> EventIterator {
    EventIterator::new()
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Key(KeyState),
    Mouse(MouseState)
}

pub struct EventIterator;

impl EventIterator {
    pub fn new() -> Self {
        EventIterator
    }
}

impl Iterator for EventIterator {
    type Item = (EventFlags, Event);

    fn next(&mut self) -> Option<(EventFlags, Event)> {
        check_for_event(KEY | MOUSE)
    }
}
