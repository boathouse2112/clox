
#include <stdlib.h>
#include "memory.h"

/// Determine a new, larger capacity for our ArrayList.
int grow_capacity(int old_capacity) {
    if (old_capacity < 8) {
        return 8;
    } else {
        return old_capacity * 2;
    }
}

void *reallocate(void *ptr, size_t old_size, size_t new_size) {
    (void) old_size; // TODO -- use old_size

    if (new_size == 0) {
        if (ptr != NULL) {
            free(ptr);
        }
        return NULL;
    }

    void *result = realloc(ptr, new_size);
    if (result == NULL) exit(1); // realloc can fail
    return result;
}
