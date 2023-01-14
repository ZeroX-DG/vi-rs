use serde::Serialize;
use std::fmt::Write;

#[derive(Serialize)]
pub(crate) struct Metadata<'a> {
    pub(crate) input_file: &'a str,
}

#[macro_export]
macro_rules! gen_test {
    ($test_method:ident, $name:tt, $path: tt) => {
        #[test]
        fn $name() {
            let contents = include_str!($path);
            let mut settings = insta::Settings::clone_current();
            settings.set_snapshot_path("../testdata/output/");
            settings.set_info(&crate::shared::Metadata { input_file: $path });
            settings.bind(|| {
                insta::assert_snapshot!($test_method(contents));
            });
        }
    };
}

pub fn transform_lines<F>(input: &str, transformer: F) -> String
where
    F: Fn(&'_ str) -> String,
{
    let mut result = String::new();
    for line in input.lines() {
        let words = line.trim().split_whitespace();

        let mut transformed_line = String::new();
        for word in words {
            let transformed_words = transformer(word);
            write!(&mut transformed_line, "{} ", transformed_words).unwrap();
        }
        write!(&mut result, "{}\n", transformed_line.trim()).unwrap();
    }
    result
}
