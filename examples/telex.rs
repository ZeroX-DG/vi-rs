extern crate vi;

use vi::telex;

fn main() {
    let inputs = "hoiwx anh tifnh yeue gioosng nhuw cais cheets nuotos trooi taats car";

    let words = inputs.split(' ');

    let mut result = String::new();
    for word in words {
        telex::transform_buffer(word.chars(), &mut result);
        result.push(' ');
    }

    println!("{}", result); // prints "hỡi anh tình yêu giống như cái chết nuốt trôi tất cả"
}
