use crate::TransformResult;

/// Transform input buffer containing a single syllable to vietnamese string output using telex mode.
///
/// # Example
/// ```
/// use vi::telex::transform_buffer;
///
/// let mut result = String::new();
/// transform_buffer("vieetj".chars(), &mut result);
/// assert_eq!(result, "việt".to_owned());
/// ```
#[deprecated(since = "0.7.0", note = "please use `vi::transform_buffer` instead")]
pub fn transform_buffer<I>(buffer: I, output: &mut String) -> TransformResult
where
    I: IntoIterator<Item = char>,
{
    crate::transform_buffer(&crate::TELEX, buffer, output)
}
