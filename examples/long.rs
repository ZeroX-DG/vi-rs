extern crate vi;

use vi::vni;

fn main() {
    let inputs = "xin chao2 toi6 la2 Hung7, toi6 den961 tu72 Viet65 Nam";

    let words = inputs.split(' ');

    let mut result: Vec<String> = vec![];
    for word in words {
        let transform_result = &vni::transform_buffer(word.chars());
        result.push(transform_result.to_owned());
    }

    println!("{}", result.join(" ")); // prints "xin chào tôi là Hưng, tôi đến từ Việt Nam"
}
