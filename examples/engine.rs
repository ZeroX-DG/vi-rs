extern crate vi;

use vi::{Engine};
use vi::key::{Key, KeyType, KeyState};

fn main() {
    let mut engine = Engine::new();
    
    let input = vec![
        Key::new(KeyType::Char('v'), KeyState::Down),
        Key::new(KeyType::Char('i'), KeyState::Down),
        Key::new(KeyType::Char('e'), KeyState::Down),
        Key::new(KeyType::Char('t'), KeyState::Down),
        Key::new(KeyType::Char('5'), KeyState::Down),
        Key::new(KeyType::Char('6'), KeyState::Down),
    ];

    for key in input {
        let actions = engine.handle_key(key);
        println!("{:#?}", actions);
    }
}
