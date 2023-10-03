extern crate vi;

use vi::vni;

fn main() {
    let inputs = "anh se4 lam2, lam2 ta6t1 ca3 de963 d9uo75c che6t1 thay em";

    let words = inputs.split(' ');

    let mut result = String::new();
    for word in words {
        vni::transform_buffer(word.chars(), &mut result);
        result.push(' ');
    }

    println!("{}", result); // prints " anh sẽ làm, làm tất cả để được chết thay em"
}
