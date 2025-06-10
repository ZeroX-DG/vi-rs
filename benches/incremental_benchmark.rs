use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use vi::{methods::transform_buffer_incremental, transform_buffer, TELEX, VNI};

fn incremental_vs_batch_telex(c: &mut Criterion) {
    let test_input = "nghienge";
    
    c.bench_function("telex incremental nghienge", |b| {
        b.iter(|| {
            let mut buffer = transform_buffer_incremental(&TELEX);
            for ch in black_box(test_input.chars()) {
                buffer.push(ch);
            }
            black_box(buffer.view().to_string())
        })
    });
    
    c.bench_function("telex batch nghienge", |b| {
        b.iter(|| {
            let mut output = String::new();
            transform_buffer(&TELEX, black_box(test_input.chars()), &mut output);
            black_box(output)
        })
    });
}

fn incremental_vs_batch_vni(c: &mut Criterion) {
    let test_input = "nghieng6";
    
    c.bench_function("vni incremental nghieng6", |b| {
        b.iter(|| {
            let mut buffer = transform_buffer_incremental(&VNI);
            for ch in black_box(test_input.chars()) {
                buffer.push(ch);
            }
            black_box(buffer.view().to_string())
        })
    });
    
    c.bench_function("vni batch nghieng6", |b| {
        b.iter(|| {
            let mut output = String::new();
            transform_buffer(&VNI, black_box(test_input.chars()), &mut output);
            black_box(output)
        })
    });
}

fn incremental_character_by_character(c: &mut Criterion) {
    c.bench_function("incremental character-by-character simulation", |b| {
        b.iter(|| {
            let test_input = "nghienge";
            let mut buffer = transform_buffer_incremental(&TELEX);
            
            // Simulate real-time typing where we need the result after each character
            let mut results = Vec::new();
            for ch in black_box(test_input.chars()) {
                buffer.push(ch);
                results.push(buffer.view().to_string());
            }
            black_box(results)
        })
    });
}

fn incremental_reuse_buffer(c: &mut Criterion) {
    c.bench_function("incremental buffer reuse", |b| {
        b.iter(|| {
            let test_inputs = ["viet", "nam", "hello", "world"];
            let mut buffer = transform_buffer_incremental(&TELEX);
            let mut results = Vec::new();
            
            for input in black_box(&test_inputs) {
                buffer.clear();
                for ch in input.chars() {
                    buffer.push(ch);
                }
                results.push(buffer.view().to_string());
            }
            black_box(results)
        })
    });
}

fn incremental_long_input(c: &mut Criterion) {
    let long_input = "xin chao toi la Hung, toi den tu Viet Nam";
    
    c.bench_function("incremental long input", |b| {
        b.iter(|| {
            let mut buffer = transform_buffer_incremental(&TELEX);
            for ch in black_box(long_input.chars()) {
                buffer.push(ch);
            }
            black_box(buffer.view().to_string())
        })
    });
    
    c.bench_function("batch long input", |b| {
        b.iter(|| {
            let mut output = String::new();
            transform_buffer(&TELEX, black_box(long_input.chars()), &mut output);
            black_box(output)
        })
    });
}

fn incremental_memory_efficiency(c: &mut Criterion) {
    c.bench_function("incremental memory usage", |b| {
        b.iter(|| {
            let mut buffer = transform_buffer_incremental(&TELEX);
            
            // Simulate typing and clearing multiple times
            for _ in 0..100 {
                for ch in black_box("vietj".chars()) {
                    buffer.push(ch);
                }
                black_box(buffer.view().to_string());
                buffer.clear();
            }
        })
    });
}

criterion_group!(
    benches,
    incremental_vs_batch_telex,
    incremental_vs_batch_vni,
    incremental_character_by_character,
    incremental_reuse_buffer,
    incremental_long_input,
    incremental_memory_efficiency
);
criterion_main!(benches);
