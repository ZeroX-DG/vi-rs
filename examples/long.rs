extern crate vi;

use vi::vni;

fn main() {
    let inputs = "xin chao2 toi6 la2 Hung7, toi6 den961 tu72 Viet65 Nam";

    let words = inputs.split(' ');

    let mut result = String::new();
    for word in words {
        vni::transform_buffer(word.chars(), &mut result);
        result.push(' ');
    }

    println!("{}", result); // prints "xin chào tôi là Hưng, tôi đến từ Việt Nam"
}
