use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

pub fn telex_benchmark(c: &mut Criterion) {
    c.bench_function("telex vieejt", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::TELEX, black_box("vieejt".chars()), &mut output)
        })
    });

    c.bench_function("telex ddaay", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::TELEX, black_box("ddaay".chars()), &mut output)
        })
    });

    c.bench_function("telex jjjjjjjjjjjjjj", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::TELEX, black_box("jjjjjjjjjjjjjj".chars()), &mut output)
        })
    });

    c.bench_function("telex jj", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::TELEX, black_box("jj".chars()), &mut output)
        })
    });

    c.bench_function("telex nghienge", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::TELEX, black_box("nghienge".chars()), &mut output)
        })
    });

    c.bench_function("telex ddaaysf", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::TELEX, black_box("ddaaysf".chars()), &mut output)
        })
    });
}

pub fn vni_benchmark(c: &mut Criterion) {
    c.bench_function("vni viet65", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::VNI, black_box("viet65".chars()), &mut output)
        })
    });

    c.bench_function("vni day96", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::VNI, black_box("ddaay".chars()), &mut output)
        })
    });

    c.bench_function("vni 1111111111111111", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::VNI, black_box("1111111111111111".chars()), &mut output)
        })
    });

    c.bench_function("vni 11", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::VNI, black_box("11".chars()), &mut output)
        })
    });

    c.bench_function("vni nghieng6", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::VNI, black_box("nghieng6".chars()), &mut output)
        })
    });

    c.bench_function("vni day9612", |b| {
        b.iter(|| {
            let mut output = String::new();
            vi::transform_buffer(&vi::VNI, black_box("day9612".chars()), &mut output)
        })
    });
}

criterion_group!(benches, telex_benchmark, vni_benchmark);
criterion_main!(benches);
