mod engine;
mod keyboard;

use keyboard::get_keyboard;

use engine::{Engine, Action};

fn main() {
    let mut keyboard = get_keyboard();
    let mut engine = Engine::new();
    
    keyboard.init();

    loop {
        if let Some(key) = keyboard.wait_for_key() {
            println!("{:?}", key);
            let instructions = engine.handle_key(key);
            for instruction in instructions {
                match instruction {
                    Action::Forward(amount) => keyboard.forward(amount),
                    Action::Back(amount) => keyboard.back(amount),
                    Action::Insert(ch) => keyboard.insert(ch),
                    Action::Backspace(amount) => keyboard.backspace(amount),
                }
            }
        }
    }
}
