mod key;
mod engine;
mod keyboard;

use engine::{Engine, Action};
use keyboard::Keyboard;

#[cfg(target_os = "linux")]
use keyboard::linux::EvdevUinputKeyboard;

fn main() {
    let mut engine = Engine::new();
    let mut keyboard: Option<Box<dyn Keyboard>> = None;

    if cfg!(target_os = "linux") {
        let input_device = evdev::Device::open(&"/dev/input/event4".to_owned())
            .unwrap();
        let output_device = uinput::default().unwrap()
            .name("test").unwrap()
            .event(uinput::event::Keyboard::All).unwrap()
            .create().unwrap();

        keyboard = Some(Box::new(EvdevUinputKeyboard::new(input_device, output_device)));
    }

    if let Some(mut keyboard) = keyboard {
        loop {
            let key = keyboard.wait_for_key();
            let instructions = engine.handle_key(key);
            if !instructions.is_empty() {
                println!("{:?}", instructions);
            }
            for instruction in instructions {
                match instruction {
                    Action::Insert(text) => keyboard.insert(text),
                    Action::Backspace(amount) => keyboard.backspace(amount),
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}
