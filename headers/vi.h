#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum AccentStyle {
  Old,
  New,
} AccentStyle;

typedef enum InputMethod {
  Telex,
  Vni,
} InputMethod;

/**
 * An incremental buffer for character-by-character Vietnamese text transformation.
 *
 * This structure allows for incremental processing of Vietnamese input, where characters
 * are added one at a time and the transformation result can be viewed at any point.
 * This is particularly useful for input method engines that need to display preview
 * text as the user types.
 *
 * # Memory Optimization
 *
 * The buffer caches the current syllable state and transformation history to avoid
 * recomputing the entire transformation on each character addition.
 *
 * # Examples
 *
 * ```
 * use vi::methods::transform_buffer_incremental;
 *
 * let mut buffer = transform_buffer_incremental(&vi::TELEX);
 *
 * buffer.push('v');
 * assert_eq!(buffer.view(), "v");
 *
 * buffer.push('i');
 * assert_eq!(buffer.view(), "vi");
 *
 * buffer.push('e');
 * assert_eq!(buffer.view(), "vie");
 *
 * buffer.push('t');
 * assert_eq!(buffer.view(), "viet");
 *
 * buffer.push('s');
 * assert_eq!(buffer.view(), "viét");
 * ```
 */
typedef struct IncrementalBuffer IncrementalBuffer;

/**
 * Transforms a C string using the specified input method and accent style.
 *
 * @param input_str The input C string to transform. Must be valid UTF-8.
 * @param method The input method to use (Telex or Vni).
 * @param accent_style The accent style to use (Old or New).
 * @return A newly allocated C string with the transformed text.
 *         The caller is responsible for freeing this string using `vi_free_string`.
 *         Returns NULL if the input string is NULL or not valid UTF-8, or if memory allocation fails.
 *
 * # Safety
 *
 * The `input_str` pointer must be a valid, null-terminated C string. If it's non-NULL,
 * it must point to readable memory that contains valid UTF-8 data. The memory
 * referenced by `input_str` must be valid for the duration of this call.
 */
char *vi_transform_string(const char *input_str,
                          enum InputMethod method,
                          enum AccentStyle accent_style);

/**
 * Creates a new incremental buffer for the given input method and accent style.
 *
 * @param method The input method to use (Telex or Vni).
 * @param accent_style The accent style to use (Old or New).
 * @return A pointer to the newly created incremental buffer.
 *         The caller is responsible for destroying this buffer using `vi_incremental_buffer_destroy`.
 *         Returns NULL if memory allocation fails.
 */
struct IncrementalBuffer *vi_incremental_buffer_create(enum InputMethod method,
                                                       enum AccentStyle accent_style);

/**
 * Pushes a character to the incremental buffer.
 *
 * @param buffer_ptr A pointer to the incremental buffer. If NULL, no operation is performed.
 * @param ch The character to push. Note: this is a Rust `char`, which is a 4-byte Unicode scalar value.
 *           For C, this typically means passing a `wchar_t` or ensuring the `char` can represent the intended Unicode codepoint if it's multi-byte.
 *           The `cbindgen` tool will typically map Rust `char` to `uint32_t` or `wchar_t` depending on the C standard and target.
 *           Ensure the character encoding is handled correctly on the C side.
 *
 * # Safety
 *
 * The `buffer_ptr` must be a valid pointer to an `IncrementalBuffer` obtained from
 * `vi_incremental_buffer_create` and not yet destroyed by `vi_incremental_buffer_destroy`.
 * If `buffer_ptr` is NULL, the function will return early.
 */
void vi_incremental_buffer_push(struct IncrementalBuffer *buffer_ptr,
                                uint32_t ch);

/**
 * Returns a C string representing the current view of the incremental buffer.
 *
 * IMPORTANT: The current implementation allocates a new C string on each call.
 * The caller is responsible for freeing this string using `vi_free_string`.
 *
 * @param buffer_ptr A pointer to the incremental buffer.
 * @return A newly allocated C string with the current transformed text in the buffer.
 *         The caller MUST free this string using `vi_free_string`.
 *         Returns NULL if the buffer_ptr is NULL or if memory allocation fails.
 *
 * # Safety
 *
 * The `buffer_ptr` must be a valid pointer to an `IncrementalBuffer` obtained from
 * `vi_incremental_buffer_create` and not yet destroyed. If `buffer_ptr` is NULL,
 * the function returns NULL. The returned string, if not NULL, must be freed by the caller
 * using `vi_free_string`.
 */
char *vi_incremental_buffer_view(const struct IncrementalBuffer *buffer_ptr);

/**
 * Destroys an incremental buffer and frees its associated memory.
 *
 * @param buffer_ptr A pointer to the incremental buffer to destroy.
 *                   If NULL, no operation is performed.
 *
 * # Safety
 *
 * If `buffer_ptr` is non-NULL, it must be a valid pointer to an `IncrementalBuffer`
 * that was allocated by `vi_incremental_buffer_create` and has not been previously destroyed.
 * After this call, the pointer is no longer valid and must not be used.
 */
void vi_incremental_buffer_destroy(struct IncrementalBuffer *buffer_ptr);

/**
 * Frees a C string that was allocated by `vi_transform_string` or `vi_incremental_buffer_view`.
 *
 * @param s The C string to free. If s is NULL, no operation is performed.
 *
 * # Safety
 *
 * If `s` is non-NULL, it must be a valid pointer to a C string that was previously
 * returned by `vi_transform_string` or `vi_incremental_buffer_view`.
 * After this call, the pointer `s` is no longer valid and must not be used.
 * Calling this function with a pointer not obtained from the mentioned functions,
 * or calling it more than once on the same pointer, leads to undefined behavior.
 */
void vi_free_string(char *s);
