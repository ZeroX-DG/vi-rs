use x11::xlib;
use super::{Keyboard};
use crate::engine::{keycodes, PhysicKey, KeyState};
use std::{ptr, slice};

pub struct KeyboardHandler {
    display: *mut xlib::Display
}

impl KeyboardHandler {
    pub fn new() -> Self {
        unsafe {
            let display: *mut xlib::Display = xlib::XOpenDisplay(ptr::null());
            Self {
                display
            }
        }
    }

    fn snoop_all_windows(&self, mut root: xlib::Window, type_: i64) {
        let mut parent: xlib::Window = 0;
        let mut children: *mut xlib::Window = ptr::null_mut();
        let mut nchildren: u32 = 0;

        unsafe {
            let status = xlib::XQueryTree(
                self.display,
                root,
                &mut root,
                &mut parent,
                &mut children,
                &mut nchildren
            );

            if status == 0 {
                println!("Can't query window tree");
                return;
            }

            if nchildren == 0 {
                return;
            }

            xlib::XSelectInput(self.display, root, type_);

            let children_slice = slice::from_raw_parts_mut(
                children,
                nchildren as usize
            );
            for i in 0..nchildren {
                let child = children_slice[i as usize];
                xlib::XSelectInput(self.display, child, type_);
                self.snoop_all_windows(child, type_);
            }
            xlib::XFree(children as *mut std::ffi::c_void);
        }
    }
}

impl Keyboard for KeyboardHandler {
    fn init(&mut self) {
        unsafe {
            let root = xlib::XDefaultRootWindow(self.display);
            self.snoop_all_windows(root, xlib::KeyPressMask);
        }
    }
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
    fn wait_for_key(&self) -> Option<PhysicKey> {
        let mut event: xlib::XEvent = xlib::XEvent {
            type_: 0
        };

        loop {
            unsafe {
                xlib::XNextEvent(self.display, &mut event);
                match event.get_type() {
                    xlib::KeyPress => {
                        if event.key.send_event == 0 {
                            break
                        }
                    },
                    _ => continue
                }
            }
        }

        unsafe {
            let keyev: xlib::XKeyEvent = event.key;
            match keyev.keycode {
                keycodes::KeyA => Some(PhysicKey {
                    keycode: 'a' as u32,
                    cap: None,
                    state: KeyState::KeyPress
                }),
                _ => None
            }
        }
    }
}

