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

macro_rules! gen_test {
    ($name:tt, $path: tt) => {
        #[test]
        fn $name() {
            let contents = include_str!($path);
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("../testdata/output/");
            settings.bind(|| {
                insta::assert_snapshot!(snapshot_transform(contents));
            });
        }
    }
}
gen_test!(simple_telex, "../testdata/input/simple_telex.txt");
