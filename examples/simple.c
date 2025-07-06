#include <stdio.h>
#include <stdlib.h>
#include <string.h>
// This will be generated in the target directory
// You might need to adjust the include path based on your build setup
#include "vi.h"

int main() {
    // Example 1: Simple transformation
    printf("Example 1: Simple transformation\n");
    const char *input1 = "viet5 nam"; // "việt nam" with VNI
    char *output1 = vi_transform_string(input1, Vni, New);
    if (output1) {
        printf("Input: \"%s\" (VNI, New Accent)\nOutput: \"%s\"\n", input1, output1);
        vi_free_string(output1);
    } else {
        printf("Transformation failed for input: %s\n", input1);
    }

    const char *input2 = "chuwongw"; // "chương" with Telex
    char *output2 = vi_transform_string(input2, Telex, New);
    if (output2) {
        printf("Input: \"%s\" (Telex, New Accent)\nOutput: \"%s\"\n", input2, output2);
        vi_free_string(output2);
    } else {
        printf("Transformation failed for input: %s\n", input2);
    }
    printf("\n");

    // Example 2: Incremental transformation
    printf("Example 2: Incremental transformation (Telex, New Accent)\n");
    IncrementalBuffer *buffer = vi_incremental_buffer_create(Telex, New);
    if (!buffer) {
        printf("Failed to create incremental buffer\n");
        return 1;
    }

    const char *incremental_input = "vieetj"; // Should become "việt"
    printf("Input sequence: %s\n", incremental_input);

    for (size_t i = 0; i < strlen(incremental_input); ++i) {
        // Note: Rust char is uint32_t. For simplicity, this example uses ASCII chars.
        // For full Unicode, ensure correct wchar_t or uint32_t conversion if needed.
        vi_incremental_buffer_push(buffer, incremental_input[i]);
        char *view = vi_incremental_buffer_view(buffer);
        if (view) {
            printf("Pushed '%c': Current view: \"%s\"\n", incremental_input[i], view);
            vi_free_string(view); // Important: free the string from vi_incremental_buffer_view
        } else {
            printf("Pushed '%c': Failed to get view\n", incremental_input[i]);
        }
    }

    vi_incremental_buffer_destroy(buffer);
    printf("\n");

    // Example 3: VNI incremental
    printf("Example 3: Incremental transformation (VNI, Old Accent)\n");
    buffer = vi_incremental_buffer_create(Vni, Old);
    if (!buffer) {
        printf("Failed to create incremental buffer\n");
        return 1;
    }
    const char *vni_incremental_input = "viet65 nam"; // "việt nam"
     printf("Input sequence: %s\n", vni_incremental_input);

    for (size_t i = 0; i < strlen(vni_incremental_input); ++i) {
        if (vni_incremental_input[i] == ' ') {
            // Simulate word break: effectively process space by showing current buffer, then clear for next word (optional)
            // Or, if spaces are handled by transformer, just push it.
            // For this library, space isn't a transformation char, so it just appends.
            // If you want separate words, you'd create a new buffer or clear it.
            // Here, we just push it to see how it's handled.
             vi_incremental_buffer_push(buffer, vni_incremental_input[i]);
        } else {
            vi_incremental_buffer_push(buffer, vni_incremental_input[i]);
        }
        char *view = vi_incremental_buffer_view(buffer);
        if (view) {
            printf("Pushed '%c': Current view: \"%s\"\n", vni_incremental_input[i], view);
            vi_free_string(view);
        } else {
             printf("Pushed '%c': Failed to get view\n", vni_incremental_input[i]);
        }
    }
    vi_incremental_buffer_destroy(buffer);

    return 0;
}
