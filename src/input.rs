use std::mem;
use std::str;

use bindings::ffi;
use bindings::{CStr, c_bool, c_char, c_uint, keycode_from_native};


/// Deprecated. Use `tcod::input::Mouse` instead.
pub type MouseState = Mouse;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum KeyCode {
    NoKey = ffi::TCOD_keycode_t::TCODK_NONE as u32,
    Escape = ffi::TCOD_keycode_t::TCODK_ESCAPE as u32,
    Backspace = ffi::TCOD_keycode_t::TCODK_BACKSPACE as u32,
    Tab = ffi::TCOD_keycode_t::TCODK_TAB as u32,
    Enter = ffi::TCOD_keycode_t::TCODK_ENTER as u32,
    Shift = ffi::TCOD_keycode_t::TCODK_SHIFT as u32,
    Control = ffi::TCOD_keycode_t::TCODK_CONTROL as u32,
    Alt = ffi::TCOD_keycode_t::TCODK_ALT as u32,
    Pause = ffi::TCOD_keycode_t::TCODK_PAUSE as u32,
    CapsLock = ffi::TCOD_keycode_t::TCODK_CAPSLOCK as u32,
    PageUp = ffi::TCOD_keycode_t::TCODK_PAGEUP as u32,
    PageDown = ffi::TCOD_keycode_t::TCODK_PAGEDOWN as u32,
    End = ffi::TCOD_keycode_t::TCODK_END as u32,
    Home = ffi::TCOD_keycode_t::TCODK_HOME as u32,
    Up = ffi::TCOD_keycode_t::TCODK_UP as u32,
    Left = ffi::TCOD_keycode_t::TCODK_LEFT as u32,
    Right = ffi::TCOD_keycode_t::TCODK_RIGHT as u32,
    Down = ffi::TCOD_keycode_t::TCODK_DOWN as u32,
    PrintScreen = ffi::TCOD_keycode_t::TCODK_PRINTSCREEN as u32,
    Insert = ffi::TCOD_keycode_t::TCODK_INSERT as u32,
    Delete = ffi::TCOD_keycode_t::TCODK_DELETE as u32,
    LeftWin = ffi::TCOD_keycode_t::TCODK_LWIN as u32,
    RightWin = ffi::TCOD_keycode_t::TCODK_RWIN as u32,
    Apps = ffi::TCOD_keycode_t::TCODK_APPS as u32,
    // The numbers on the alphanum section of the keyboard
    Number0 = ffi::TCOD_keycode_t::TCODK_0 as u32,
    Number1 = ffi::TCOD_keycode_t::TCODK_1 as u32,
    Number2 = ffi::TCOD_keycode_t::TCODK_2 as u32,
    Number3 = ffi::TCOD_keycode_t::TCODK_3 as u32,
    Number4 = ffi::TCOD_keycode_t::TCODK_4 as u32,
    Number5 = ffi::TCOD_keycode_t::TCODK_5 as u32,
    Number6 = ffi::TCOD_keycode_t::TCODK_6 as u32,
    Number7 = ffi::TCOD_keycode_t::TCODK_7 as u32,
    Number8 = ffi::TCOD_keycode_t::TCODK_8 as u32,
    Number9 = ffi::TCOD_keycode_t::TCODK_9 as u32,
    // The numbers on the numeric keypad
    NumPad0 = ffi::TCOD_keycode_t::TCODK_KP0 as u32,
    NumPad1 = ffi::TCOD_keycode_t::TCODK_KP1 as u32,
    NumPad2 = ffi::TCOD_keycode_t::TCODK_KP2 as u32,
    NumPad3 = ffi::TCOD_keycode_t::TCODK_KP3 as u32,
    NumPad4 = ffi::TCOD_keycode_t::TCODK_KP4 as u32,
    NumPad5 = ffi::TCOD_keycode_t::TCODK_KP5 as u32,
    NumPad6 = ffi::TCOD_keycode_t::TCODK_KP6 as u32,
    NumPad7 = ffi::TCOD_keycode_t::TCODK_KP7 as u32,
    NumPad8 = ffi::TCOD_keycode_t::TCODK_KP8 as u32,
    NumPad9 = ffi::TCOD_keycode_t::TCODK_KP9 as u32,
    NumPadAdd = ffi::TCOD_keycode_t::TCODK_KPADD as u32,
    NumPadSubtract = ffi::TCOD_keycode_t::TCODK_KPSUB as u32,
    NumPadDivide = ffi::TCOD_keycode_t::TCODK_KPDIV as u32,
    NumPadMultiply = ffi::TCOD_keycode_t::TCODK_KPMUL as u32,
    NumPadDecimal = ffi::TCOD_keycode_t::TCODK_KPDEC as u32,
    NumPadEnter = ffi::TCOD_keycode_t::TCODK_KPENTER as u32,
    F1 = ffi::TCOD_keycode_t::TCODK_F1 as u32,
    F2 = ffi::TCOD_keycode_t::TCODK_F2 as u32,
    F3 = ffi::TCOD_keycode_t::TCODK_F3 as u32,
    F4 = ffi::TCOD_keycode_t::TCODK_F4 as u32,
    F5 = ffi::TCOD_keycode_t::TCODK_F5 as u32,
    F6 = ffi::TCOD_keycode_t::TCODK_F6 as u32,
    F7 = ffi::TCOD_keycode_t::TCODK_F7 as u32,
    F8 = ffi::TCOD_keycode_t::TCODK_F8 as u32,
    F9 = ffi::TCOD_keycode_t::TCODK_F9 as u32,
    F10 = ffi::TCOD_keycode_t::TCODK_F10 as u32,
    F11 = ffi::TCOD_keycode_t::TCODK_F11 as u32,
    F12 = ffi::TCOD_keycode_t::TCODK_F12 as u32,
    NumLock = ffi::TCOD_keycode_t::TCODK_NUMLOCK as u32,
    ScrollLock = ffi::TCOD_keycode_t::TCODK_SCROLLLOCK as u32,
    Spacebar = ffi::TCOD_keycode_t::TCODK_SPACE as u32,
    Char = ffi::TCOD_keycode_t::TCODK_CHAR as u32,
    Text = ffi::TCOD_keycode_t::TCODK_TEXT as u32,
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

    text: [c_char; 32],
}

impl Key {
    pub fn text(&self) -> &str {
        unsafe {
            CStr::from_ptr(&self.text[0] as *const c_char).to_str().unwrap()
        }
    }
}

impl From<ffi::TCOD_key_t> for Key {
    fn from(tcod_key: ffi::TCOD_key_t) -> Key {
        Key {
            code: keycode_from_native(tcod_key.vk).unwrap(),
            text: tcod_key.text,
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
        const KEY_PRESSED  = ffi::TCOD_key_status_t::TCOD_KEY_PRESSED as c_uint,
        const KEY_RELEASED = ffi::TCOD_key_status_t::TCOD_KEY_RELEASED as c_uint,
    }
}

bitflags! {
    flags EventFlags: c_uint {
        const KEY_PRESS     = ffi::TCOD_event_t::TCOD_EVENT_KEY_PRESS as c_uint,
        const KEY_RELEASE   = ffi::TCOD_event_t::TCOD_EVENT_KEY_RELEASE as c_uint,
        const KEY           = ffi::TCOD_event_t::TCOD_EVENT_KEY as c_uint,
        const MOUSE_MOVE    = ffi::TCOD_event_t::TCOD_EVENT_MOUSE_MOVE as c_uint,
        const MOUSE_PRESS   = ffi::TCOD_event_t::TCOD_EVENT_MOUSE_PRESS as c_uint,
        const MOUSE_RELEASE = ffi::TCOD_event_t::TCOD_EVENT_MOUSE_RELEASE as c_uint,
        const MOUSE         = ffi::TCOD_event_t::TCOD_EVENT_MOUSE as c_uint,
        const ANY           = ffi::TCOD_event_t::TCOD_EVENT_ANY as c_uint,
    }
}

pub fn check_for_event(event_mask: EventFlags) -> Option<(EventFlags, Event)> {
    let mut c_key_state: mem::MaybeUninit<ffi::TCOD_key_t> = mem::MaybeUninit::uninit();
    let mut c_mouse_state: mem::MaybeUninit<ffi::TCOD_mouse_t> = mem::MaybeUninit::uninit();

    let event = unsafe {
        ffi::TCOD_sys_check_for_event(event_mask.bits() as i32,
                                      c_key_state.as_mut_ptr(), c_mouse_state.as_mut_ptr())
    };

    let c_key_state = unsafe { c_key_state.assume_init() };
    let c_mouse_state = unsafe { c_mouse_state.assume_init() };

    let ret_flag = match event {
        ffi::TCOD_event_t::TCOD_EVENT_KEY_PRESS => KEY_PRESS,
        ffi::TCOD_event_t::TCOD_EVENT_KEY_RELEASE => KEY_RELEASE,
        ffi::TCOD_event_t::TCOD_EVENT_KEY => KEY,
        ffi::TCOD_event_t::TCOD_EVENT_MOUSE => MOUSE,
        ffi::TCOD_event_t::TCOD_EVENT_MOUSE_MOVE => MOUSE_MOVE,
        ffi::TCOD_event_t::TCOD_EVENT_MOUSE_PRESS => MOUSE_PRESS,
        ffi::TCOD_event_t::TCOD_EVENT_MOUSE_RELEASE => MOUSE_RELEASE,
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
