use super::{Keyboard};
use crate::engine::{PhysicKey, KeyState, KeyCap};
use x11::xlib::{
    self as xlib,
    Display, XOpenDisplay, XDefaultRootWindow,
    XEvent, XKeyEvent, XNextEvent, XEventsQueued, XPeekEvent,
    KeyPressMask, FocusChangeMask, KeyReleaseMask,
    XSelectInput, XGetInputFocus, XGetKeyboardMapping, XKeysymToKeycode, XFree
};
use x11::xtest::XTestFakeKeyEvent;
use x11::keysym;
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
        unsafe {
            let keycode = XKeysymToKeycode(
                self.display,
                keysym::XK_BackSpace.into()
            );
            for _ in 0..amount {
                XTestFakeKeyEvent(self.display, keycode.into(), 1, 0);
                XTestFakeKeyEvent(self.display, keycode.into(), 0, 0);
            }
        }
    }
    fn insert(&self, ch: char) {
        unsafe {
            let keycode = 0x11af;
            //let keycode = XKeysymToKeycode(self.display, ch as u64);
            XTestFakeKeyEvent(self.display, keycode, 1, 0);
            XTestFakeKeyEvent(self.display, keycode, 0, 0);
        }
    }
    fn wait_for_key(&mut self) -> PhysicKey {
        let mut ev: XEvent = unsafe { mem::zeroed() };
        let mask = KeyPressMask | KeyReleaseMask | FocusChangeMask;
        unsafe {
            let mut root = XDefaultRootWindow(self.display);
            XGetInputFocus(self.display, &mut root, &mut xlib::RevertToParent);
            XSelectInput(self.display, root, mask);
            loop {
                XNextEvent(self.display, &mut ev);
                match ev.get_type() {
                    xlib::KeyPress => {
                        if ev.key.send_event == 0 && ev.key.time != 0 {
                            let mut keysyms_per_keycode_return: i32 = 0;
                            let keysym = XGetKeyboardMapping(
                                self.display,
                                ev.key.keycode as u8,
                                1,
                                &mut keysyms_per_keycode_return
                            );

                            let mut need_break = false;

                            match *keysym as u32 {
                                keysym::XK_Shift_L | keysym::XK_Shift_R => {
                                    self.is_shift_down = true;
                                },
                                keysym::XK_Control_L | keysym::XK_Control_R => {
                                    self.is_ctrl_down = true;
                                },
                                keysym::XK_Caps_Lock => {
                                    self.is_capslock_down = !self.is_capslock_down;
                                },
                                _ => need_break = true
                            }

                            XFree(keysym as *mut std::ffi::c_void);
                            
                            if need_break {
                                break
                            }
                        }
                    },
                    xlib::KeyRelease => {
                        let mut is_auto_repeat = false;
                        // QueuedAfterReading = 1
                        if XEventsQueued(self.display, 1) == 1 {
                            let mut xev: XEvent = mem::zeroed();
                            XPeekEvent(self.display, &mut xev);
                            if xev.get_type() == xlib::KeyPress &&
                                xev.key.time == ev.key.time &&
                                xev.key.keycode == ev.key.keycode {
                                is_auto_repeat = true;
                            }
                        }
                        
                        if ev.key.send_event == 0 && !is_auto_repeat {
                            let mut keysyms_per_keycode_return: i32 = 0;
                            let keysym = XGetKeyboardMapping(
                                self.display,
                                ev.key.keycode as u8,
                                1,
                                &mut keysyms_per_keycode_return
                            );

                            let mut need_break = false;
                            match *keysym as u32 {
                                keysym::XK_Shift_L | keysym::XK_Shift_R => {
                                    self.is_shift_down = false;
                                },
                                keysym::XK_Control_L | keysym::XK_Control_R => {
                                    self.is_ctrl_down = false;
                                },
                                keysym::XK_Caps_Lock => {
                                    // do nothing
                                    // simply to make the key invisible to the
                                    // engine handler
                                },
                                _ => need_break = true
                            };

                            XFree(keysym as *mut std::ffi::c_void);

                            if need_break {
                                break
                            }
                        }
                    },
                    xlib::FocusOut => {
                        XSelectInput(self.display, root, 0);
                        XGetInputFocus(
                            self.display,
                            &mut root,
                            &mut xlib::RevertToParent
                        );
                        XSelectInput(self.display, root, mask);
                    },
                    _ => continue
                }
            }
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

