use x11::xlib;
use super::{Keyboard};
use crate::engine::{keycodes, PhysicKey, KeyState};
use std::{ptr, slice};

pub struct KeyboardHandler {
    display: *mut xlib::Display,
    focus_window: xlib::Window
}

impl KeyboardHandler {
    pub fn new() -> Self {
        unsafe {
            let display: *mut xlib::Display = xlib::XOpenDisplay(ptr::null());
            let mut focus_window: xlib::Window = 0;
            xlib::XGetInputFocus(
                display,
                &mut focus_window,
                &mut xlib::RevertToPointerRoot
            );
            xlib::XSelectInput(
                display,
                focus_window,
                xlib::KeyPressMask|xlib::FocusChangeMask
            );
            Self {
                display,
                focus_window 
            }
        }
    }
}

impl Keyboard for KeyboardHandler {
    fn forward(&self, amount: usize) {}
    fn back(&self, amount: usize) {}
    fn backspace(&self, amount: usize) {
        unsafe {
            x11::xtest::XTestFakeKeyEvent(
               self.display,
               keycodes::Key_BACKSPACE,
               1,
               0
            );
        }
    }
    fn insert(&self, ch: char) {}
    fn wait_for_key(&mut self) -> Option<PhysicKey> {
        let mut event: xlib::XEvent = xlib::XEvent {
            type_: 0
        };

        unsafe {
            let root = xlib::XDefaultRootWindow(self.display);
            loop {
                xlib::XNextEvent(self.display, &mut event);
                match event.get_type() {
                    xlib::KeyPress => {
                        if event.key.send_event == 0 {
                            break
                        }
                    },
                    xlib::FocusOut => {
                        if self.focus_window != root {
                            xlib::XSelectInput(
                                self.display,
                                self.focus_window,
                                0
                            );
                        }
                        xlib::XGetInputFocus(
                            self.display,
                            &mut self.focus_window,
                            &mut xlib::RevertToPointerRoot
                        );
                        if self.focus_window as i32 == xlib::PointerRoot {
                            self.focus_window = root;
                        }
                        xlib::XSelectInput(
                            self.display,
                            self.focus_window,
                            xlib::KeyPressMask | xlib::FocusChangeMask
                        );
                    },
                    _ => continue
                }
            }
            let keyev: xlib::XKeyEvent = event.key;
            Some(PhysicKey {
                keycode: keyev.keycode,
                cap: None,
                state: KeyState::KeyPress
            })
        }
    }
}

