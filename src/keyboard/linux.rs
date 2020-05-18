use super::{Keyboard};
use crate::engine::{PhysicKey, KeyState, KeyCap};
use x11::xlib::{
    self as xlib,
    Display, XOpenDisplay, XDefaultRootWindow, XCloseDisplay,
    XEvent, XKeyEvent, XNextEvent, XEventsQueued, XPeekEvent, XSync, XFree,
    KeyPressMask, FocusChangeMask, KeyReleaseMask,
    XSelectInput, XGetInputFocus, XGetKeyboardMapping, XKeysymToKeycode, XKeycodeToKeysym,
    XDisplayKeycodes, XChangeKeyboardMapping, CurrentTime, XFlush
};
use x11::xtest::{XTestFakeKeyEvent};
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

    fn find_keycode_to_remap(&self) -> i32 {
        let mut min_key_code = 0;
        let mut max_key_code = 0;
        unsafe {
            XDisplayKeycodes(self.display, &mut min_key_code, &mut max_key_code);
            let mut current_key_code = min_key_code;
            while current_key_code < max_key_code {
                let keysym = XKeycodeToKeysym(self.display, current_key_code as u8, 0);
                if keysym == xlib::NoSymbol as u64 {
                    return current_key_code
                }
                current_key_code += 1;
            }
        }
        min_key_code
    }

    fn remap_scratch_keycode(&self, scratch_keycode: i32, keysym: u64) {
        unsafe {
            let mut keysyms = [keysym];
            XChangeKeyboardMapping(
                self.display,
                scratch_keycode,
                1,
                keysyms.as_mut_ptr(),
                1
            );
            XSync(self.display, 0);
        }
    }

    fn char_to_keysym(&self, ch: char) -> u32 {
        // this is a fix for missing vietnamese character
        // see: https://www.cl.cam.ac.uk/~mgk25/ucs/keysymdef.h
        match ch {
            'Ạ' => 0x1001ea0,
            'ạ' => 0x1001ea1,
            'Ả' => 0x1001ea2,
            'ả' => 0x1001ea3,
            'Ấ' => 0x1001ea4,
            'ấ' => 0x1001ea5,
            'Ầ' => 0x1001ea6,
            'ầ' => 0x1001ea7,
            'Ẩ' => 0x1001ea8,
            'ẩ' => 0x1001ea9,
            'Ẫ' => 0x1001eaa,
            'ẫ' => 0x1001eab,
            'Ậ' => 0x1001eac,
            'ậ' => 0x1001ead,
            'Ắ' => 0x1001eae,
            'ắ' => 0x1001eaf,
            'Ằ' => 0x1001eb0,
            'ằ' => 0x1001eb1,
            'Ẳ' => 0x1001eb2,
            'ẳ' => 0x1001eb3,
            'Ẵ' => 0x1001eb4,
            'ẵ' => 0x1001eb5,
            'Ặ' => 0x1001eb6,
            'ặ' => 0x1001eb7,
            'Ẹ' => 0x1001eb8,
            'ẹ' => 0x1001eb9,
            'Ẻ' => 0x1001eba,
            'ẻ' => 0x1001ebb,
            'Ẽ' => 0x1001ebc,
            'ẽ' => 0x1001ebd,
            'Ế' => 0x1001ebe,
            'ế' => 0x1001ebf,
            'Ề' => 0x1001ec0,
            'ề' => 0x1001ec1,
            'Ể' => 0x1001ec2,
            'ể' => 0x1001ec3,
            'Ễ' => 0x1001ec4,
            'ễ' => 0x1001ec5,
            'Ệ' => 0x1001ec6,
            'ệ' => 0x1001ec7,
            'Ỉ' => 0x1001ec8,
            'ỉ' => 0x1001ec9,
            'Ị' => 0x1001eca,
            'ị' => 0x1001ecb,
            'Ọ' => 0x1001ecc,
            'ọ' => 0x1001ecd,
            'Ỏ' => 0x1001ece,
            'ỏ' => 0x1001ecf,
            'Ố' => 0x1001ed0,
            'ố' => 0x1001ed1,
            'Ồ' => 0x1001ed2,
            'ồ' => 0x1001ed3,
            'Ổ' => 0x1001ed4,
            'ổ' => 0x1001ed5,
            'Ỗ' => 0x1001ed6,
            'ỗ' => 0x1001ed7,
            'Ộ' => 0x1001ed8,
            'ộ' => 0x1001ed9,
            'Ớ' => 0x1001eda,
            'ớ' => 0x1001edb,
            'Ờ' => 0x1001edc,
            'ờ' => 0x1001edd,
            'Ở' => 0x1001ede,
            'ở' => 0x1001edf,
            'Ỡ' => 0x1001ee0,
            'ỡ' => 0x1001ee1,
            'Ợ' => 0x1001ee2,
            'ợ' => 0x1001ee3,
            'Ụ' => 0x1001ee4,
            'ụ' => 0x1001ee5,
            'Ủ' => 0x1001ee6,
            'ủ' => 0x1001ee7,
            'Ứ' => 0x1001ee8,
            'ứ' => 0x1001ee9,
            'Ừ' => 0x1001eea,
            'ừ' => 0x1001eeb,
            'Ử' => 0x1001eec,
            'ử' => 0x1001eed,
            'Ữ' => 0x1001eee,
            'ữ' => 0x1001eef,
            'Ự' => 0x1001ef0,
            'ự' => 0x1001ef1,
            'Ỵ' => 0x1001ef4,
            'ỵ' => 0x1001ef5,
            'Ỷ' => 0x1001ef6,
            'ỷ' => 0x1001ef7,
            'Ỹ' => 0x1001ef8,
            'ỹ' => 0x1001ef9,
            'Ơ' => 0x10001a0,
            'ơ' => 0x10001a1,
            'Ư' => 0x10001af,
            'ư' => 0x10001b0,
            'Đ' => 0x01d0,
            'đ' => 0x01f0,
            _ => ch as u32
        }
    }
}

impl Keyboard for KeyboardHandler {
    fn backspace(&self, amount: usize) {
        unsafe {
            let keycode = XKeysymToKeycode(
                self.display,
                keysym::XK_BackSpace.into()
            );
            for _ in 0..amount {
                XTestFakeKeyEvent(self.display, keycode.into(), 1, CurrentTime);
                // prevent XNextEvent to catch this fake key
                XSync(self.display, 1); 
                XFlush(self.display);
                XTestFakeKeyEvent(self.display, keycode.into(), 0, CurrentTime);
                XSync(self.display, 1);
                XFlush(self.display);
            }
        }
    }
    fn insert(&self, ch: char) {
        let keysym = self.char_to_keysym(ch);
        unsafe {
            let mut keycode: i32 = XKeysymToKeycode(self.display, keysym.into()) as i32;
            let need_remap = keycode > 255 || keycode < 8;
            if need_remap {
                keycode = self.find_keycode_to_remap();
                self.remap_scratch_keycode(keycode, keysym.into());
            }
            XTestFakeKeyEvent(self.display, keycode as u32, 1, CurrentTime);
            XSync(self.display, 1);
            XFlush(self.display);
            XTestFakeKeyEvent(self.display, keycode as u32, 0, CurrentTime);
            XSync(self.display, 1);
            XFlush(self.display);
            if need_remap {
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    let display = XOpenDisplay(std::ptr::null());
                    XChangeKeyboardMapping(
                        display,
                        keycode,
                        1,
                        &mut (xlib::NoSymbol as u64),
                        1
                    );
                    XSync(display, 0);
                    XCloseDisplay(display);
                });
            }
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

