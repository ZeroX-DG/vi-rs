extern crate vi;

use vi::vni;

fn main() {
    let inputs = vec![vec!['v', 'i', 'e', 't', '5', '6'], vec!['n', 'a', 'm']];

    let mut result = String::new();
    for input in inputs {
        let transform_result = &vni::transform_buffer(input.iter().cloned());
        result.push_str(transform_result);
        result.push(' ');
    }

    println!("{}", result); // prints "việt nam "
}
