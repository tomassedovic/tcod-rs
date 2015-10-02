use std::mem;

use bindings::ffi;
use bindings::{c_bool, c_uint, keycode_from_u32};


/// Deprecated. Use `tcod::input::Mouse` instead.
pub type MouseState = Mouse;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum KeyCode {
    NoKey = ffi::TCODK_NONE as isize,
    Escape = ffi::TCODK_ESCAPE as isize,
    Backspace = ffi::TCODK_BACKSPACE as isize,
    Tab = ffi::TCODK_TAB as isize,
    Enter = ffi::TCODK_ENTER as isize,
    Shift = ffi::TCODK_SHIFT as isize,
    Control = ffi::TCODK_CONTROL as isize,
    Alt = ffi::TCODK_ALT as isize,
    Pause = ffi::TCODK_PAUSE as isize,
    CapsLock = ffi::TCODK_CAPSLOCK as isize,
    PageUp = ffi::TCODK_PAGEUP as isize,
    PageDown = ffi::TCODK_PAGEDOWN as isize,
    End = ffi::TCODK_END as isize,
    Home = ffi::TCODK_HOME as isize,
    Up = ffi::TCODK_UP as isize,
    Left = ffi::TCODK_LEFT as isize,
    Right = ffi::TCODK_RIGHT as isize,
    Down = ffi::TCODK_DOWN as isize,
    PrintScreen = ffi::TCODK_PRINTSCREEN as isize,
    Insert = ffi::TCODK_INSERT as isize,
    Delete = ffi::TCODK_DELETE as isize,
    LeftWin = ffi::TCODK_LWIN as isize,
    RightWin = ffi::TCODK_RWIN as isize,
    Apps = ffi::TCODK_APPS as isize,
    // The numbers on the alphanum section of the keyboard
    Number0 = ffi::TCODK_0 as isize,
    Number1 = ffi::TCODK_1 as isize,
    Number2 = ffi::TCODK_2 as isize,
    Number3 = ffi::TCODK_3 as isize,
    Number4 = ffi::TCODK_4 as isize,
    Number5 = ffi::TCODK_5 as isize,
    Number6 = ffi::TCODK_6 as isize,
    Number7 = ffi::TCODK_7 as isize,
    Number8 = ffi::TCODK_8 as isize,
    Number9 = ffi::TCODK_9 as isize,
    // The numbers on the numeric keypad
    NumPad0 = ffi::TCODK_KP0 as isize,
    NumPad1 = ffi::TCODK_KP1 as isize,
    NumPad2 = ffi::TCODK_KP2 as isize,
    NumPad3 = ffi::TCODK_KP3 as isize,
    NumPad4 = ffi::TCODK_KP4 as isize,
    NumPad5 = ffi::TCODK_KP5 as isize,
    NumPad6 = ffi::TCODK_KP6 as isize,
    NumPad7 = ffi::TCODK_KP7 as isize,
    NumPad8 = ffi::TCODK_KP8 as isize,
    NumPad9 = ffi::TCODK_KP9 as isize,
    NumPadAdd = ffi::TCODK_KPADD as isize,
    NumPadSubtract = ffi::TCODK_KPSUB as isize,
    NumPadDivide = ffi::TCODK_KPDIV as isize,
    NumPadMultiply = ffi::TCODK_KPMUL as isize,
    NumPadDecimal = ffi::TCODK_KPDEC as isize,
    NumPadEnter = ffi::TCODK_KPENTER as isize,
    F1 = ffi::TCODK_F1 as isize,
    F2 = ffi::TCODK_F2 as isize,
    F3 = ffi::TCODK_F3 as isize,
    F4 = ffi::TCODK_F4 as isize,
    F5 = ffi::TCODK_F5 as isize,
    F6 = ffi::TCODK_F6 as isize,
    F7 = ffi::TCODK_F7 as isize,
    F8 = ffi::TCODK_F8 as isize,
    F9 = ffi::TCODK_F9 as isize,
    F10 = ffi::TCODK_F10 as isize,
    F11 = ffi::TCODK_F11 as isize,
    F12 = ffi::TCODK_F12 as isize,
    NumLock = ffi::TCODK_NUMLOCK as isize,
    ScrollLock = ffi::TCODK_SCROLLLOCK as isize,
    Spacebar = ffi::TCODK_SPACE as isize,
    Char = ffi::TCODK_CHAR as isize,
}

impl Default for KeyCode {
    fn default() -> Self {
        KeyCode::NoKey
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Key {
    pub code: KeyCode,
    pub printable: char,
    pub pressed: bool,
    pub left_alt: bool,
    pub left_ctrl: bool,
    pub right_alt: bool,
    pub right_ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub ctrl: bool,
}

impl From<ffi::TCOD_key_t> for Key {
    fn from(tcod_key: ffi::TCOD_key_t) -> Key {
        Key {
            code: keycode_from_u32(tcod_key.vk).unwrap(),
            printable: tcod_key.c as u8 as char,
            pressed: tcod_key.pressed != 0,
            left_alt: tcod_key.lalt != 0,
            left_ctrl: tcod_key.lctrl != 0,
            right_alt: tcod_key.ralt != 0,
            right_ctrl: tcod_key.rctrl != 0,
            shift: tcod_key.shift != 0,
            alt: tcod_key.lalt != 0 || tcod_key.ralt != 0,
            ctrl: tcod_key.lctrl != 0 || tcod_key.rctrl != 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Mouse {
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
    let mut c_key_state: ffi::TCOD_key_t = unsafe { mem::uninitialized() };
    let mut c_mouse_state: ffi::TCOD_mouse_t = unsafe { mem::uninitialized() };

    let event = unsafe {
        ffi::TCOD_sys_check_for_event(event_mask.bits() as i32,
                                      &mut c_key_state, &mut c_mouse_state)
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
        Some(Event::Key(c_key_state.into()))
    } else if ret_flag.intersects(MOUSE_MOVE|MOUSE_PRESS|MOUSE_RELEASE|MOUSE) {
        Some(Event::Mouse(Mouse {
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

    ret_event.map(|event| (ret_flag, event))
}

pub fn events() -> EventIterator {
    EventIterator::new()
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Key(Key),
    Mouse(Mouse)
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
