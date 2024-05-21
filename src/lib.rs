//! A Vietnamese typing composition engine.
//!
//! VI implement support for Vietnamese text composition using the two most common methods: Telex and VNI.
//! The two main methods of this library is `telex::transform_buffer` and `vni::transform_buffer` but
//! a range of useful utility methods and constants for Vietnamese text manipulation are also exposed.
//!
//! # Aspirations
//!
//! - Minimal to zero configuration. The engine should have a default configurations that fit with most usecases.
//! - Support all transformation behaviours & orders. Adding a tonemark during or after typing a word should just work.
//! - Be as fast as possible.
//! - Be as simple as possible.
//!
//! # Example
//!
//! ```
//! use vi::vni;
//! let inputs = vec![vec!['v', 'i', 'e', 't', '5', '6'], vec!['n', 'a', 'm']];
//! let mut result = String::new();
//! for input in inputs {
//!     vni::transform_buffer(input.iter().cloned(), &mut result);
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
//! - **Tone mark are placed in the new accent:** hoà instead of hòa
//! - **`w` in telex will insert `ư`:** so `chuw` or `chw` will produce `chư`
pub mod editing;
pub mod maps;
pub mod parsing;
pub mod processor;
pub mod telex;
pub mod util;
pub mod validation;
pub mod vni;
pub mod word;

/// A result of a buffer transformation.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TransformResult {
    /// Indicates whether a tone mark has been removed during the transformation.
    tone_mark_removed: bool,
    /// Indicates whether a letter modification has been removed during the transformation.
    letter_modification_removed: bool,
}
