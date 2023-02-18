mod shared;

fn snapshot_transform(lines: &str) -> String {
    shared::transform_lines(lines, |word| {
        let mut trasformed_word = String::new();
        vi::telex::transform_buffer(word.chars(), &mut trasformed_word);
        trasformed_word
    })
}

macro_rules! gen_test_telex {
    ($name:tt, $path: tt) => {
        gen_test!(snapshot_transform, $name, $path);
    };
}
gen_test_telex!(simple_telex, "../testdata/input/simple_telex.txt");
gen_test_telex!(
    non_vietnamese_telex,
    "../testdata/input/non_vietnamese_telex.txt"
);
gen_test_telex!(
    bogo_engine_telex,
    "../testdata/input/bogo_engine_telex.txt"
);