use vi::{transform_buffer, VNI};

fn main() {
    let inputs = "xin chao2 toi6 la2 Hung7, toi6 den961 tu72 Viet65 Nam";

    let words = inputs.split(' ');

    let mut result = String::new();
    for word in words {
        transform_buffer(&VNI, word.chars(), &mut result);
        result.push(' ');
    }

    println!("{}", result); // prints "xin chào tôi là Hưng, tôi đến từ Việt Nam"
}
