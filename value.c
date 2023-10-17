#include <printf.h>
#include "value.h"
#include "memory.h"

void value_print(Value value) {
    printf("%g", value);
}

void value_array_init(ValueArray *arr) {
    arr->length = 0;
    arr->capacity = 0;
    arr->data = NULL;
}

void value_array_push(ValueArray *arr, Value value) {
    if (arr->capacity < arr->length + 1) {
        int old_capacity = arr->capacity;
        arr->capacity = grow_capacity(old_capacity);
        arr->data = GROW_ARRAY(Value, arr->data, old_capacity, arr->capacity);
    }

    arr->data[arr->length] = value;
    arr->length += 1;
}

void value_array_free(ValueArray *arr) {
    FREE_ARRAY(Value, arr->data, arr->capacity);
}
