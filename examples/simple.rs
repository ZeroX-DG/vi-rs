extern crate vi;

fn main() {
    let inputs = vec![vec!['v', 'i', 'e', 't', '5', '6'], vec!['n', 'a', 'm']];

    let mut result = String::new();
    for input in inputs {
        vi::transform_buffer(&vi::VNI, input.iter().cloned(), &mut result);
        result.push(' ');
    }

    println!("{}", result); // prints "việt nam "
}
