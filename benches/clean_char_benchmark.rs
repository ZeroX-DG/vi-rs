use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use vi::util::clean_char;

fn benchmark_clean_char_vietnamese_chars(c: &mut Criterion) {
    let vietnamese_chars = [
        'á', 'à', 'ả', 'ã', 'ạ', 'ă', 'ắ', 'ằ', 'ẳ', 'ẵ', 'ặ', 'â', 'ấ', 'ầ', 'ẩ', 'ẫ', 'ậ', 'é',
        'è', 'ẻ', 'ẽ', 'ẹ', 'ê', 'ế', 'ề', 'ể', 'ễ', 'ệ', 'í', 'ì', 'ỉ', 'ĩ', 'ị', 'ó', 'ò', 'ỏ',
        'õ', 'ọ', 'ô', 'ố', 'ồ', 'ổ', 'ỗ', 'ộ', 'ơ', 'ớ', 'ờ', 'ở', 'ỡ', 'ợ', 'ú', 'ù', 'ủ', 'ũ',
        'ụ', 'ư', 'ứ', 'ừ', 'ử', 'ữ', 'ự', 'ý', 'ỳ', 'ỷ', 'ỹ', 'ỵ', 'đ', 'Á', 'À', 'Ả', 'Ã', 'Ạ',
        'Ă', 'Ắ', 'Ằ', 'Ẳ', 'Ẵ', 'Ặ', 'Â', 'Ấ', 'Ầ', 'Ẩ', 'Ẫ', 'Ậ', 'É', 'È', 'Ẻ', 'Ẽ', 'Ẹ', 'Ê',
        'Ế', 'Ề', 'Ể', 'Ễ', 'Ệ', 'Í', 'Ì', 'Ỉ', 'Ĩ', 'Ị', 'Ó', 'Ò', 'Ỏ', 'Õ', 'Ọ', 'Ô', 'Ố', 'Ồ',
        'Ổ', 'Ỗ', 'Ộ', 'Ơ', 'Ớ', 'Ờ', 'Ở', 'Ỡ', 'Ợ', 'Ú', 'Ù', 'Ủ', 'Ũ', 'Ụ', 'Ư', 'Ứ', 'Ừ', 'Ử',
        'Ữ', 'Ự', 'Ý', 'Ỳ', 'Ỷ', 'Ỹ', 'Ỵ', 'Đ',
    ];

    c.bench_function("clean_char_vietnamese_chars", |b| {
        b.iter(|| {
            for &ch in vietnamese_chars.iter() {
                black_box(clean_char(black_box(ch)));
            }
        })
    });
}

fn benchmark_clean_char_non_vietnamese_chars(c: &mut Criterion) {
    let non_vietnamese_chars = [
        'b', 'c', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
        'z', 'B', 'C', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W',
        'X', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ', '.', ',', '!', '?', '-',
        '_', '(', ')', '[', ']', '{', '}',
    ];

    c.bench_function("clean_char_non_vietnamese_chars", |b| {
        b.iter(|| {
            for &ch in non_vietnamese_chars.iter() {
                black_box(clean_char(black_box(ch)));
            }
        })
    });
}

fn benchmark_clean_char_mixed_text(c: &mut Criterion) {
    let mixed_text = "Xin chào! Tôi là một người Việt Nam. Hello, I am Vietnamese.";

    c.bench_function("clean_char_mixed_text", |b| {
        b.iter(|| {
            for ch in mixed_text.chars() {
                black_box(clean_char(black_box(ch)));
            }
        })
    });
}

fn benchmark_clean_char_const_evaluation(c: &mut Criterion) {
    c.bench_function("clean_char_const_evaluation", |b| {
        b.iter(|| {
            // These should be evaluated at compile time
            const CLEANED_1: char = clean_char('á');
            const CLEANED_2: char = clean_char('Ế');
            const CLEANED_3: char = clean_char('ự');
            const CLEANED_4: char = clean_char('Đ');

            black_box((CLEANED_1, CLEANED_2, CLEANED_3, CLEANED_4));
        })
    });
}

criterion_group!(
    benches,
    benchmark_clean_char_vietnamese_chars,
    benchmark_clean_char_non_vietnamese_chars,
    benchmark_clean_char_mixed_text,
    benchmark_clean_char_const_evaluation
);
criterion_main!(benches);
