extern crate vi;

use rustyline::DefaultEditor;
use vi::{telex, vni};

// A REPL for testing transformation result.
fn main() {
    let method = "telex";
    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let Ok(input) = rl.readline("(input): ") else {
            break;
        };

        let mut result = String::new();

        for word in input.split_whitespace() {
            if method == "telex" {
                telex::transform_buffer(word.chars(), &mut result)
            } else {
                vni::transform_buffer(word.chars(), &mut result)
            };
            result.push(' ');
        }

        println!("(output): {}", result);
    }
}
