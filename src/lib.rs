//! A Vietnamese typing composition engine.
//!
//! VI implement support for Vietnamese text composition using the two most common methods: Telex and VNI.
//! The two main methods of this library is `telex::transform_buffer` and `vni::transform_buffer` but
//! a range of useful utility methods and constants for Vietnamese text manipulation are also exposed.
//!
//! # Aspirations
//!
//! - Minimal to zero configuration. The engine should have a default configurations that fit with most usecases.
//! - Support all transformation behaviours & orders. Adding a tonemark during or after typing a syllable should just work.
//! - Be as fast as possible.
//! - Be as simple as possible.
//!
//! # Example
//!
//! ```
//! let inputs = vec![vec!['v', 'i', 'e', 't', '5', '6'], vec!['n', 'a', 'm']];
//! let mut result = String::new();
//! for input in inputs {
//!     vi::transform_buffer(&vi::VNI, input.iter().cloned(), &mut result);
//!     result.push(' ');
//! }
//! println!("{}", result); // prints "việt nam "
//! ```
//!
//! # Rules
//!
//! VI aims to be as lean as possible, focusing on only the useful features and main use-cases. Therefore, the engine
//! implemented these rules by default with no way of configuring them:
//!
//! - **Tone mark are placed in the new accent:** `hoà` instead of `hòa`
//! - **`w` in telex will insert `ư`:** so `chuw` or `chw` will produce `chư`
//!
//! Although, should you need to customise any behaviour, you can create your custom typing methods. See: [`methods`].
pub mod editing;
pub mod maps;
pub mod methods;
pub mod parsing;
pub mod processor;
pub mod syllable;
#[deprecated(since = "0.7.0")]
pub mod telex;
pub mod util;
pub mod validation;
#[deprecated(since = "0.7.0")]
pub mod vni;

pub use methods::*;
pub use syllable::Syllable;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
pub enum InputMethod {
    Telex,
    Vni,
}

#[repr(C)]
pub enum AccentStyle {
    Old,
    New,
}

/// Transforms a C string using the specified input method and accent style.
///
/// @param input_str The input C string to transform. Must be valid UTF-8.
/// @param method The input method to use (Telex or Vni).
/// @param accent_style The accent style to use (Old or New).
/// @return A newly allocated C string with the transformed text.
///         The caller is responsible for freeing this string using `vi_free_string`.
///         Returns NULL if the input string is NULL or not valid UTF-8, or if memory allocation fails.
#[no_mangle]
pub extern "C" fn vi_transform_string(
    input_str: *const c_char,
    method: InputMethod,
    accent_style: AccentStyle,
) -> *mut c_char {
    if input_str.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(input_str) };
    let rust_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(), // Invalid UTF-8
    };

    let definition = match method {
        InputMethod::Telex => &TELEX,
        InputMethod::Vni => &VNI,
    };

    let style = match accent_style {
        AccentStyle::Old => crate::processor::AccentStyle::Old,
        AccentStyle::New => crate::processor::AccentStyle::New,
    };

    let mut result = String::new();
    transform_buffer_with_style(definition, style, rust_str.chars(), &mut result);

    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(), // Result contains null byte
    }
}

// --- Incremental Buffer C API ---

/// Creates a new incremental buffer for the given input method and accent style.
///
/// @param method The input method to use (Telex or Vni).
/// @param accent_style The accent style to use (Old or New).
/// @return A pointer to the newly created incremental buffer.
///         The caller is responsible for destroying this buffer using `vi_incremental_buffer_destroy`.
///         Returns NULL if memory allocation fails.
#[no_mangle]
pub extern "C" fn vi_incremental_buffer_create(
    method: InputMethod,
    accent_style: AccentStyle,
) -> *mut IncrementalBuffer<'static> {
    let definition = match method {
        InputMethod::Telex => &TELEX,
        InputMethod::Vni => &VNI,
    };
    let style = match accent_style {
        AccentStyle::Old => crate::processor::AccentStyle::Old,
        AccentStyle::New => crate::processor::AccentStyle::New,
    };
    let buffer = IncrementalBuffer::new_with_style(definition, style);
    Box::into_raw(Box::new(buffer))
}

/// Pushes a character to the incremental buffer.
///
/// @param buffer_ptr A pointer to the incremental buffer. If NULL, no operation is performed.
/// @param ch The character to push. Note: this is a Rust `char`, which is a 4-byte Unicode scalar value.
///           For C, this typically means passing a `wchar_t` or ensuring the `char` can represent the intended Unicode codepoint if it's multi-byte.
///           The `cbindgen` tool will typically map Rust `char` to `uint32_t` or `wchar_t` depending on the C standard and target.
///           Ensure the character encoding is handled correctly on the C side.
#[no_mangle]
pub extern "C" fn vi_incremental_buffer_push(buffer_ptr: *mut IncrementalBuffer, ch: char) {
    if buffer_ptr.is_null() {
        return;
    }
    let buffer = unsafe { &mut *buffer_ptr };
    buffer.push(ch);
}

/// Returns a C string representing the current view of the incremental buffer.
///
/// IMPORTANT: The current implementation allocates a new C string on each call.
/// The caller is responsible for freeing this string using `vi_free_string`.
///
/// @param buffer_ptr A pointer to the incremental buffer.
/// @return A newly allocated C string with the current transformed text in the buffer.
///         The caller MUST free this string using `vi_free_string`.
///         Returns NULL if the buffer_ptr is NULL or if memory allocation fails.
#[no_mangle]
pub extern "C" fn vi_incremental_buffer_view(buffer_ptr: *const IncrementalBuffer) -> *mut c_char {
    if buffer_ptr.is_null() {
        // It's generally better to return a CString for safety, but for a view,
        // we might return null or an empty string CString if the buffer is null.
        // For now, let's assume buffer_ptr is valid as per typical C API contracts,
        // or the caller handles null.
        // If we must return a valid c_char, an empty static CString could be an option.
        return std::ptr::null_mut();
    }
    // The user must call vi_free_string on this result. This is documented in the function header.
    match CString::new(unsafe { &*buffer_ptr }.view()) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => {
            // This case should ideally not happen if buffer.view() returns a valid string
            // without null bytes. If it can, returning null_mut() is safer.
            CString::new("").unwrap().into_raw() // Fallback to empty string
        }
    }
}


/// Destroys an incremental buffer and frees its associated memory.
///
/// @param buffer_ptr A pointer to the incremental buffer to destroy.
///                   If NULL, no operation is performed.
#[no_mangle]
pub extern "C" fn vi_incremental_buffer_destroy(buffer_ptr: *mut IncrementalBuffer) {
    if buffer_ptr.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(buffer_ptr);
    }
}

/// Frees a C string that was allocated by `vi_transform_string` or `vi_incremental_buffer_view`.
///
/// @param s The C string to free. If s is NULL, no operation is performed.
#[no_mangle]
pub extern "C" fn vi_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}
