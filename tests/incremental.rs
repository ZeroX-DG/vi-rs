use vi::{
    methods::{transform_buffer_incremental, transform_buffer_incremental_with_style, IncrementalBuffer},
    processor::AccentStyle,
    transform_buffer, TELEX, VNI,
};

#[test]
fn test_incremental_buffer_basic_telex() {
    let mut buffer = transform_buffer_incremental(&TELEX);
    
    // Test empty buffer
    assert_eq!(buffer.view(), "");
    assert!(buffer.is_empty());
    assert_eq!(buffer.len(), 0);
    assert_eq!(buffer.input(), &[]);
    
    // Test single character
    let _ = buffer.push('v');
    assert_eq!(buffer.view(), "v");
    assert!(!buffer.is_empty());
    assert_eq!(buffer.len(), 1);
    assert_eq!(buffer.input(), &['v']);

    // Test building up a word
    let _ = buffer.push('i');
    assert_eq!(buffer.view(), "vi");
    assert_eq!(buffer.len(), 2);

    let _ = buffer.push('e');
    assert_eq!(buffer.view(), "vie");
    assert_eq!(buffer.len(), 3);

    let _ = buffer.push('t');
    assert_eq!(buffer.view(), "viet");
    assert_eq!(buffer.len(), 4);

    // Test tone mark addition
    let _ = buffer.push('s');
    assert_eq!(buffer.view(), "viét");
    assert_eq!(buffer.len(), 5);
    assert_eq!(buffer.input(), &['v', 'i', 'e', 't', 's']);
}

#[test]
fn test_incremental_buffer_basic_vni() {
    let mut buffer = transform_buffer_incremental(&VNI);
    
    let _ = buffer.push('v');
    let _ = buffer.push('i');
    let _ = buffer.push('e');
    let _ = buffer.push('t');
    let _ = buffer.push('6');
    let _ = buffer.push('5');
    
    assert_eq!(buffer.view(), "việt");
    assert_eq!(buffer.input(), &['v', 'i', 'e', 't', '6', '5']);
}

#[test]
fn test_incremental_buffer_clear() {
    let mut buffer = transform_buffer_incremental(&TELEX);
    
    let _ = buffer.push('v');
    let _ = buffer.push('i');
    let _ = buffer.push('e');
    let _ = buffer.push('t');
    let _ = buffer.push('s');

    assert_eq!(buffer.view(), "viét");
    assert!(!buffer.is_empty());
    assert_eq!(buffer.len(), 5);

    buffer.clear();
    
    assert_eq!(buffer.view(), "");
    assert!(buffer.is_empty());
    assert_eq!(buffer.len(), 0);
    assert_eq!(buffer.input(), &[]);
}

#[test]
fn test_incremental_buffer_tone_removal() {
    let mut buffer = transform_buffer_incremental(&TELEX);
    
    let _ = buffer.push('v');
    let _ = buffer.push('i');
    let _ = buffer.push('e');
    let _ = buffer.push('t');
    let _ = buffer.push('s');
    assert_eq!(buffer.view(), "viét");

    // Remove tone mark with 'z'
    let result = buffer.push('z');
    assert_eq!(buffer.view(), "viet");
    assert!(result.tone_mark_removed);
    assert!(buffer.result().tone_mark_removed);
}

#[test]
fn test_incremental_buffer_letter_modification() {
    let mut buffer = transform_buffer_incremental(&TELEX);
    
    let _ = buffer.push('a');
    let _ = buffer.push('a');
    assert_eq!(buffer.view(), "â");

    buffer.clear();

    let _ = buffer.push('o');
    let _ = buffer.push('o');
    assert_eq!(buffer.view(), "ô");

    buffer.clear();

    let _ = buffer.push('u');
    let _ = buffer.push('w');
    assert_eq!(buffer.view(), "ư");
}

#[test]
fn test_incremental_buffer_complex_word() {
    let mut buffer = transform_buffer_incremental(&TELEX);
    
    // Build "nghiêng" character by character
    let chars = ['n', 'g', 'h', 'i', 'e', 'e', 'n', 'g'];
    for ch in chars {
        let _ = buffer.push(ch);
    }
    
    assert_eq!(buffer.view(), "nghiêng");
    assert_eq!(buffer.input(), &chars);
}

#[test]
fn test_incremental_buffer_w_insertion() {
    let mut buffer = transform_buffer_incremental(&TELEX);
    
    // Test 'w' insertion behavior
    let _ = buffer.push('w');
    assert_eq!(buffer.view(), "ư");

    // Test 'w' after 'w' should replace with 'w'
    let _ = buffer.push('w');
    assert_eq!(buffer.view(), "w");
}

#[test]
fn test_incremental_buffer_accent_style() {
    let mut buffer_new = transform_buffer_incremental_with_style(&TELEX, AccentStyle::New);
    let mut buffer_old = transform_buffer_incremental_with_style(&TELEX, AccentStyle::Old);
    
    // Build "hoà" vs "hòa"
    for ch in ['h', 'o', 'a', 's'] {
        let _ = buffer_new.push(ch);
        let _ = buffer_old.push(ch);
    }
    
    assert_eq!(buffer_new.view(), "hoá");
    assert_eq!(buffer_old.view(), "hóa");
}

#[test]
fn test_incremental_vs_batch_consistency() {
    let test_cases = [
        "viet65",
        "nghienge",
        "ddaaysf",
        "chuw",
        "hoaas",
        "tuyet",
    ];
    
    for test_case in test_cases {
        // Test with TELEX
        let mut incremental_result = String::new();
        let mut buffer = transform_buffer_incremental(&TELEX);
        
        for ch in test_case.chars() {
            let _ = buffer.push(ch);
        }
        incremental_result.push_str(buffer.view());
        
        let mut batch_result = String::new();
        transform_buffer(&TELEX, test_case.chars(), &mut batch_result);
        
        assert_eq!(
            incremental_result, batch_result,
            "TELEX: Incremental and batch results differ for input '{}'",
            test_case
        );
        
        // Test with VNI
        let mut incremental_result = String::new();
        let mut buffer = transform_buffer_incremental(&VNI);
        
        for ch in test_case.chars() {
            let _ = buffer.push(ch);
        }
        incremental_result.push_str(buffer.view());
        
        let mut batch_result = String::new();
        transform_buffer(&VNI, test_case.chars(), &mut batch_result);
        
        assert_eq!(
            incremental_result, batch_result,
            "VNI: Incremental and batch results differ for input '{}'",
            test_case
        );
    }
}

#[test]
fn test_incremental_buffer_non_vietnamese_chars() {
    let mut buffer = transform_buffer_incremental(&TELEX);
    
    // Test mixing Vietnamese and non-Vietnamese characters
    let _ = buffer.push('h');
    let _ = buffer.push('e');
    let _ = buffer.push('l');
    let _ = buffer.push('l');
    let _ = buffer.push('o');

    assert_eq!(buffer.view(), "hello");

    buffer.clear();

    // Test numbers and symbols
    let _ = buffer.push('1');
    let _ = buffer.push('2');
    let _ = buffer.push('3');
    let _ = buffer.push('!');
    
    assert_eq!(buffer.view(), "123!");
}

#[test]
fn test_incremental_buffer_constructor_variants() {
    let buffer1 = IncrementalBuffer::new(&TELEX);
    let buffer2 = transform_buffer_incremental(&TELEX);
    let buffer3 = IncrementalBuffer::new_with_style(&TELEX, AccentStyle::default());
    let buffer4 = transform_buffer_incremental_with_style(&TELEX, AccentStyle::default());
    
    // All should start empty
    assert_eq!(buffer1.view(), "");
    assert_eq!(buffer2.view(), "");
    assert_eq!(buffer3.view(), "");
    assert_eq!(buffer4.view(), "");
}
