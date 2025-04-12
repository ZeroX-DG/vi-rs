extern crate vi;

fn main() {
    let inputs = "ra tajp hoas nhaf baf thuyr mua hoa hoef";

    let words = inputs.split(' ');

    let mut result = String::new();
    for word in words {
        vi::transform_buffer_with_style(
            &vi::TELEX,
            vi::processor::AccentStyle::Old,
            word.chars(),
            &mut result,
        );
        result.push(' ');
    }

    println!("{}", result); // prints "ra tạp hóa nhà bà thủy mua hoa hòe"
}
