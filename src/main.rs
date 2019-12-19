mod engine;
mod keyboard;

use keyboard::get_keyboard;

use engine::{Engine, Action};

fn main() {
    let mut keyboard = get_keyboard();
    let mut engine = Engine::new();
    loop {
        let key = keyboard.wait_for_key();
        let instructions = engine.handle_key(key);
        if !instructions.is_empty() {
            println!("{:?}", instructions);
        }
        for instruction in instructions {
            match instruction {
                Action::Insert(ch) => keyboard.insert(ch),
                Action::Backspace(amount) => keyboard.backspace(amount),
            }
        }
    }
}
