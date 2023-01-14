extern crate vi;

use vi::vni;

fn main() {
    let inputs = vec![vec!['v', 'i', 'e', 't', '5', '6'], vec!['n', 'a', 'm']];

    let mut result = String::new();
    let mut transformed_word = String::new();
    for input in inputs {
        vni::transform_buffer(input.iter().cloned(), &mut transformed_word);
        result.push_str(&transformed_word);
        result.push(' ');
        transformed_word.clear();
    }

    println!("{}", result); // prints "việt nam "
}
