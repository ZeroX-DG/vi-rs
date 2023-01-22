mod shared;

fn snapshot_transform(lines: &str) -> String {
    shared::transform_lines(lines, |word| {
        let mut trasformed_word = String::new();
        vi::vni::transform_buffer(word.chars(), &mut trasformed_word);
        trasformed_word
    })
}

macro_rules! gen_test_vi {
    ($name:tt, $path: tt) => {
        gen_test!(snapshot_transform, $name, $path);
    };
}
gen_test_vi!(simple_vni, "../testdata/input/simple_vni.txt");
gen_test_vi!(non_vietnamese_vni, "../testdata/input/non_vietnamese_vni.txt");
