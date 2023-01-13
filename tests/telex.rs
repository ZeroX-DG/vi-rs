use std::fmt::Write;

fn snapshot_transform(input: &str) -> String {
    let mut result = String::new();
    for line in input.lines() {
        let words = line.trim().split_whitespace();

        let mut transformed_line = String::new();
        for word in words {
            let transformed_words = vi::telex::transform_buffer(word.chars());
            write!(&mut transformed_line, "{} ", transformed_words).unwrap();
        }
        write!(&mut result, "{}\n", transformed_line.trim()).unwrap();
    }
    result
}

mod shared;

macro_rules! gen_test_telex {
    ($name:tt, $path: tt) => {
        gen_test!(snapshot_transform, $name, $path);
    }
}
gen_test_telex!(simple_telex, "../testdata/input/simple_telex.txt");
