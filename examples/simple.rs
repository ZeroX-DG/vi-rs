extern crate vi;

use vi::vni;

fn main() {
    let inputs = vec![
        vec!['v', 'i', 'e', 't', '6', '5'],
        vec!['n', 'a', 'm']
    ];

    let mut result = String::new();
    for input in inputs {
        let (_, transform_result) = &vni::transform_buffer(&input);
        result.push_str(transform_result);
        result.push(' ');
    }
    
    println!("{}", result); // prints "việt nam "
}
