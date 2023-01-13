mod shared;

fn snapshot_transform(lines: &str) -> String {
    shared::transform_lines(lines, |word| vi::vni::transform_buffer(word.chars()))
}

macro_rules! gen_test_vi {
    ($name:tt, $path: tt) => {
        gen_test!(snapshot_transform, $name, $path);
    }
}
gen_test_vi!(simple_vni, "../testdata/input/simple_vni.txt");
