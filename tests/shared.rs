use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Metadata<'a> {
    pub(crate) input_file: &'a str
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
    }
}
