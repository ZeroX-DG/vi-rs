use super::Keyboard;
use super::{Key, KeyState};
use x11::xtest::XTestFakeKeyEvent;
use x11::xlib::{
    self as xlib,
    Display, XOpenDisplay, XSync,
    XKeysymToKeycode, XKeycodeToKeysym,
    XDisplayKeycodes, XChangeKeyboardMapping, CurrentTime, XFlush
};
use x11::keysym;
use std::{ptr};

pub struct EvdevX11Keyboard {
    display: *mut Display,
    input_device: evdev::Device,
}

impl EvdevX11Keyboard {
    pub fn new(input_device: evdev::Device) -> Self {
        unsafe {
            let display: *mut Display = XOpenDisplay(ptr::null());

            Self {
                display,
                input_device,
            }    
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
            'ă' => 0x01e3,
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
            'é' => 0x00e9,
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
            'ũ' => 0x03fd,
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
}

impl Keyboard for EvdevX11Keyboard {
    fn insert(&mut self, text: String) {
        let scratch_keycode = self.find_keycode_to_remap();
        for ch in text.chars() {
            let keysym = self.char_to_keysym(ch);
            unsafe {
                let mut keycode: i32 = XKeysymToKeycode(self.display, keysym.into()) as i32;
                let need_remap = keycode > 255 || keycode < 8;
                if need_remap {
                    self.remap_scratch_keycode(scratch_keycode, keysym.into());
                    XSync(self.display, 0);
                    std::thread::sleep(std::time::Duration::from_millis(50));
                    keycode = scratch_keycode;
                }
                XTestFakeKeyEvent(self.display, keycode as u32, 1, 0);
                XSync(self.display, 1);
                XTestFakeKeyEvent(self.display, keycode as u32, 0, 0);
                XSync(self.display, 1);
                if need_remap {
                    XSync(self.display, 1);
                }
            }
        }
        self.remap_scratch_keycode(scratch_keycode, xlib::NoSymbol as u64);
        unsafe {
            XSync(self.display, 0);
            XFlush(self.display);
        }
    }

    fn backspace(&mut self, amount: usize) {
        unsafe {
            let keycode = XKeysymToKeycode(
                self.display,
                keysym::XK_BackSpace.into()
            );
            for _ in 0..amount {
                XTestFakeKeyEvent(self.display, keycode.into(), 1, CurrentTime);
                // prevent XNextEvent to catch this fake key
                XSync(self.display, 0); 
                XFlush(self.display);
                XTestFakeKeyEvent(self.display, keycode.into(), 0, CurrentTime);
                XSync(self.display, 0);
                XFlush(self.display);
            }
        }
    }

    fn wait_for_key(&mut self) -> Key {
        let mut keystate: Option<KeyState> = None;
        let mut keycode: Option<u16> = None;
        loop {
            for ev in self.input_device.events_no_sync().unwrap() {
                if ev._type != 1 {
                    continue;
                }
                keystate = Some(match ev.value {
                    1 => KeyState::Down,
                    _ => KeyState::Release
                });
                keycode = Some(ev.code);
                break;
            }
            if keycode.is_some() {
                break
            }
        }
        let key_ch = match keycode.unwrap() {
            input_event_codes::KEY_A => 'a',
            input_event_codes::KEY_B => 'b',
            input_event_codes::KEY_C => 'c',
            input_event_codes::KEY_D => 'd',
            input_event_codes::KEY_E => 'e',
            input_event_codes::KEY_F => 'f',
            input_event_codes::KEY_G => 'g',
            input_event_codes::KEY_H => 'h',
            input_event_codes::KEY_I => 'i',
            input_event_codes::KEY_J => 'j',
            input_event_codes::KEY_K => 'k',
            input_event_codes::KEY_L => 'l',
            input_event_codes::KEY_M => 'm',
            input_event_codes::KEY_N => 'n',
            input_event_codes::KEY_O => 'o',
            input_event_codes::KEY_P => 'p',
            input_event_codes::KEY_Q => 'q',
            input_event_codes::KEY_R => 'r',
            input_event_codes::KEY_S => 's',
            input_event_codes::KEY_T => 't',
            input_event_codes::KEY_U => 'u',
            input_event_codes::KEY_V => 'v',
            input_event_codes::KEY_W => 'w',
            input_event_codes::KEY_X => 'x',
            input_event_codes::KEY_Y => 'y',
            input_event_codes::KEY_1 => '1',
            input_event_codes::KEY_2 => '2',
            input_event_codes::KEY_3 => '3',
            input_event_codes::KEY_4 => '4',
            input_event_codes::KEY_5 => '5',
            input_event_codes::KEY_6 => '6',
            input_event_codes::KEY_7 => '7',
            input_event_codes::KEY_8 => '8',
            input_event_codes::KEY_9 => '9',
            input_event_codes::KEY_0 => '0',
            _ => '\0'
        };
        return Key::new(key_ch, keycode.unwrap(), keystate.unwrap());
    }
}
