use super::Keyboard;
use super::{Key, KeyState};
use uinput::event::keyboard;

pub struct EvdevUinputKeyboard {
    input_device: evdev::Device,
    output_device: uinput::Device
}

pub fn key_from_char(ch: char) -> Option<keyboard::Key> {
    match ch {
        'a' => Some(keyboard::Key::A),
        'b' => Some(keyboard::Key::B),
        'c' => Some(keyboard::Key::C),
        'd' => Some(keyboard::Key::D),
        'e' => Some(keyboard::Key::E),
        'f' => Some(keyboard::Key::F),
        'g' => Some(keyboard::Key::G),
        'h' => Some(keyboard::Key::H),
        'i' => Some(keyboard::Key::I),
        'j' => Some(keyboard::Key::J),
        'k' => Some(keyboard::Key::K),
        'l' => Some(keyboard::Key::L),
        'm' => Some(keyboard::Key::M),
        'n' => Some(keyboard::Key::N),
        'o' => Some(keyboard::Key::O),
        'p' => Some(keyboard::Key::P),
        'q' => Some(keyboard::Key::Q),
        'r' => Some(keyboard::Key::R),
        's' => Some(keyboard::Key::S),
        't' => Some(keyboard::Key::T),
        'u' => Some(keyboard::Key::U),
        'v' => Some(keyboard::Key::V),
        'w' => Some(keyboard::Key::W),
        'x' => Some(keyboard::Key::X),
        'y' => Some(keyboard::Key::Y),
        'z' => Some(keyboard::Key::Z),
        '1' => Some(keyboard::Key::_1),
        '2' => Some(keyboard::Key::_2),
        '3' => Some(keyboard::Key::_3),
        '4' => Some(keyboard::Key::_4),
        '5' => Some(keyboard::Key::_5),
        '6' => Some(keyboard::Key::_6),
        '7' => Some(keyboard::Key::_7),
        '8' => Some(keyboard::Key::_8),
        '9' => Some(keyboard::Key::_9),
        '0' => Some(keyboard::Key::_0),
        _ => None
    }
}

impl EvdevUinputKeyboard {
    pub fn new(input_device: evdev::Device, output_device: uinput::Device) -> Self {
        Self {
            input_device,
            output_device
        }
    }
}

impl Keyboard for EvdevUinputKeyboard {
    fn insert(&mut self, text: String) {
        for ch in text.chars() {
            if let Some(key) = key_from_char(ch) {
                self.output_device.click(&key).unwrap();
            } else { // need composition
                //CTRL + SHIFT
                self.output_device.press(&keyboard::Key::LeftControl).unwrap();
                self.output_device.press(&keyboard::Key::LeftShift).unwrap();

                // U
                self.output_device.click(&keyboard::Key::U).unwrap();

                // Character hex
                let ascii_code = ch as u16;
                let ascii_hex = format!("{:x}", ascii_code);
                for hex in ascii_hex.chars() {
                    if let Some(hex_key) = key_from_char(hex) {
                        self.output_device.click(&hex_key).unwrap();
                    }
                }

                // release CTRL + SHIFT
                self.output_device.release(&keyboard::Key::LeftShift).unwrap();
                self.output_device.release(&keyboard::Key::LeftControl).unwrap();
            }
            self.output_device.synchronize().unwrap();
        }
    }

    fn backspace(&mut self, amount: usize) {
        for _ in 0..amount {
            self.output_device.click(&keyboard::Key::BackSpace).unwrap();
        }
        self.output_device.synchronize().unwrap();
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
