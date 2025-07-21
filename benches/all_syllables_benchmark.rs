use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs, hint::black_box, path::Path, time::Duration};

fn benchmark_all_telex_txt(c: &mut Criterion) {
    // Path to the testdata file
    let path = Path::new("testdata/input/all_telex.txt");
    let contents = fs::read_to_string(path).expect("Failed to read testdata/input/all_telex.txt");

    // Split into words (same as test snapshot logic)
    let words: Vec<&str> = contents.split_whitespace().collect();

    c.bench_function("all telex", |b| {
        b.iter(|| {
            for word in words.iter() {
                let mut output = String::new();
                vi::transform_buffer(&vi::TELEX, black_box(word.chars()), &mut output);
            }
        })
    });
}

fn benchmark_all_vni_txt(c: &mut Criterion) {
    // Path to the testdata file
    let path = Path::new("testdata/input/all_vni.txt");
    let contents = fs::read_to_string(path).expect("Failed to read testdata/input/all_vni.txt");

    // Split into words (same as test snapshot logic)
    let words: Vec<&str> = contents.split_whitespace().collect();

    c.bench_function("all vni", |b| {
        b.iter(|| {
            for word in words.iter() {
                let mut output = String::new();
                vi::transform_buffer(&vi::VNI, black_box(word.chars()), &mut output);
            }
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(30));
    targets = benchmark_all_telex_txt, benchmark_all_vni_txt
}
criterion_main!(benches);
