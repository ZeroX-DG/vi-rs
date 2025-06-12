use rustyline::DefaultEditor;

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
            let definition = if method == "telex" {
                &vi::TELEX
            } else {
                &vi::VNI
            };

            vi::transform_buffer(definition, word.chars(), &mut result);
            result.push(' ');
        }

        println!("(output): {}", result);
    }
}
