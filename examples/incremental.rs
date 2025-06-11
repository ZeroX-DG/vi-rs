use std::io::{self, Write};
use vi::{methods::transform_buffer_incremental, TELEX, VNI};

fn main() {
    println!("Vietnamese Incremental Input Demo");
    println!("=================================");
    println!();
    println!("This example demonstrates the incremental buffer feature,");
    println!("which is useful for input method engines that need to display");
    println!("preview text as the user types character by character.");
    println!();
    
    // Demo 1: Basic incremental typing
    println!("Demo 1: Building 'việt' character by character with TELEX");
    println!("---------------------------------------------------------");
    
    let mut buffer = transform_buffer_incremental(&TELEX);
    let input_chars = ['v', 'i', 'e', 't', 'j'];
    
    for ch in input_chars {
        let result = buffer.push(ch);
        println!("Input: '{}' -> Output: '{}' (tone_removed: {}, letter_mod_removed: {})", 
                 ch, buffer.view(), result.tone_mark_removed, result.letter_modification_removed);
    }
    
    println!("Final result: '{}'", buffer.view());
    println!("Input sequence: {:?}", buffer.input());
    println!();
    
    // Demo 2: VNI method
    println!("Demo 2: Building 'việt' with VNI method");
    println!("---------------------------------------");
    
    buffer.clear();
    let mut vni_buffer = transform_buffer_incremental(&VNI);
    let vni_chars = ['v', 'i', 'e', 't', '6', '5'];
    
    for ch in vni_chars {
        let _ = vni_buffer.push(ch);
        println!("Input: '{}' -> Output: '{}'", ch, vni_buffer.view());
    }
    
    println!("Final result: '{}'", vni_buffer.view());
    println!();
    
    // Demo 3: Tone mark removal
    println!("Demo 3: Tone mark removal with 'z'");
    println!("-----------------------------------");
    
    let mut buffer = transform_buffer_incremental(&TELEX);
    
    // Build "việt"
    for ch in ['v', 'i', 'e', 't', 'j'] {
        let _ = buffer.push(ch);
    }
    println!("Built word: '{}'", buffer.view());
    
    // Remove tone mark
    let result = buffer.push('z');
    println!("After 'z': '{}' (tone removed: {})", buffer.view(), result.tone_mark_removed);
    println!();
    
    // Demo 4: Letter modifications
    println!("Demo 4: Letter modifications");
    println!("----------------------------");
    
    buffer.clear();
    
    // Circumflex on 'a'
    let _ = buffer.push('a');
    println!("Input: 'a' -> Output: '{}'", buffer.view());
    let _ = buffer.push('a');
    println!("Input: 'a' -> Output: '{}' (circumflex added)", buffer.view());

    buffer.clear();

    // Horn on 'u'
    let _ = buffer.push('u');
    println!("Input: 'u' -> Output: '{}'", buffer.view());
    let _ = buffer.push('w');
    println!("Input: 'w' -> Output: '{}' (horn added)", buffer.view());
    
    println!();
    
    // Demo 5: Interactive typing simulation
    println!("Demo 5: Interactive typing simulation");
    println!("------------------------------------");
    println!("Simulating real-time typing of 'nghiêng':");
    
    buffer.clear();
    let word_chars = ['n', 'g', 'h', 'i', 'e', 'e', 'n', 'g'];
    
    for (i, ch) in word_chars.iter().enumerate() {
        let _ = buffer.push(*ch);
        println!("Step {}: '{}' -> '{}'", i + 1, ch, buffer.view());

        // Simulate typing delay
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    
    println!("Final word: '{}'", buffer.view());
    println!();
    
    // Demo 6: Buffer reuse
    println!("Demo 6: Buffer reuse for multiple words");
    println!("---------------------------------------");
    
    let words = ["viet", "nam", "hello", "world"];
    
    for word in words {
        buffer.clear();
        print!("Typing '{}': ", word);
        io::stdout().flush().unwrap();
        
        for ch in word.chars() {
            let _ = buffer.push(ch);
            print!("{} ", buffer.view());
            io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        
        println!("-> Final: '{}'", buffer.view());
    }
    
    println!();
    println!("Demo completed! The incremental buffer allows for:");
    println!("- Character-by-character processing");
    println!("- Real-time preview updates");
    println!("- Efficient state caching");
    println!("- Buffer reuse for multiple words");
    println!("- Consistent results with batch processing");
}
