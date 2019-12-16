use super::{Keyboard};
use crate::engine::{keycodes, PhysicKey, KeyState, KeyCap};
use x11::xlib::{
    self as xlib,
    Display, XOpenDisplay,
    XEvent, XKeyEvent, XNextEvent
};
use std::{ptr, mem};

pub struct KeyboardHandler {
    display: *mut Display,
    is_shift_down: bool,
    is_capslock_down: bool,
    is_ctrl_down: bool
}

impl KeyboardHandler {
    pub fn new() -> Self {
        unsafe {
            let display: *mut Display = XOpenDisplay(ptr::null());
            Self {
                display,
                is_shift_down: false,
                is_capslock_down: false,
                is_ctrl_down: false
            }
        }
    }
}

impl Keyboard for KeyboardHandler {
    fn forward(&self, amount: usize) {}
    fn back(&self, amount: usize) {}
    fn backspace(&self, amount: usize) {
    }
    fn insert(&self, ch: char) {}
    fn wait_for_key(&self) -> PhysicKey {
        let mut ev: XEvent = unsafe { mem::zeroed() };
        loop {
            unsafe {
                XNextEvent(self.display, &mut ev);
                match ev.get_type() {
                    xlib::KeyPress => {
                        if ev.key.send_event == 0 {
                            break;
                        }
                    }
                    _ => continue
                }
            }
        }

        unsafe {
            let ev_key: XKeyEvent = ev.key;
            let cap: Option<KeyCap> = if self.is_capslock_down {
                if self.is_shift_down || self.is_ctrl_down {
                    None
                } else {
                    Some(KeyCap::CapsLock)
                }
            } else if self.is_shift_down {
                if self.is_ctrl_down {
                    None
                } else {
                    Some(KeyCap::Shift)
                }
            } else {
                None
            };

            let state = if ev.get_type() == xlib::KeyPress {
                KeyState::KeyPress
            } else {
                KeyState::KeyRelease
            };

            PhysicKey {
                keycode: ev_key.keycode,
                cap,
                state
            }
        }
    }
}

