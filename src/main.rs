mod key;
mod engine;
mod keyboard;

use engine::Engine;
use keyboard::Keyboard;

#[cfg(target_os = "linux")]
use keyboard::linux::LinuxKeyboard;

#[cfg(target_os = "linux")]
fn main() {
    let input_device = evdev::Device::open(&"/dev/input/event4".to_owned())
        .unwrap();
    let output_device = uinput::default().unwrap()
		.name("test").unwrap()
		.event(uinput::event::Keyboard::All).unwrap()
		.create().unwrap();

    let mut engine = Engine::new();
    let mut keyboard = LinuxKeyboard::new(input_device, output_device);

    loop {
        let key = keyboard.wait_for_key();
        let result = engine.handle_key(key);
        if !result.is_empty() {
            println!("{:?}", result);
        }
    }
}
