#ifndef CLOX_MEMORY_H
#define CLOX_MEMORY_H

#include "common.h"

/// Grows the array to our the new count
#define GROW_ARRAY(type, ptr, old_count, new_count) \
    (type*) reallocate((ptr), sizeof(type) * (old_count), \
                       sizeof(type) * (new_count))

#define FREE_ARRAY(type, ptr, old_count) \
    reallocate(ptr, sizeof(type) * (old_count), 0)

int grow_capacity(int old_capacity);
void *reallocate(void *ptr, size_t old_size, size_t new_size);

#endif //CLOX_MEMORY_H
