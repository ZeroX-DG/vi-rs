extern crate vi;

use vi::telex;

fn main() {
    let inputs = "cuuwf";

    let words = inputs.split(' ');

    let mut result = String::new();
    for word in words {
        telex::transform_buffer(word.chars(), &mut result);
        result.push(' ');
    }

    println!("{}", result); // prints "xin chào tôi là Hưng, tôi đến từ Việt Nam"
}
