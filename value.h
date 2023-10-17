#ifndef CLOX_VALUE_H
#define CLOX_VALUE_H

#include "common.h"

/// Number line
typedef double Value;

void value_print(Value value);

typedef struct {
    int length;
    int capacity;
    Value *data;
} ValueArray;

void value_array_init(ValueArray *value_array);
void value_array_push(ValueArray *value_array, Value value);
void value_array_free(ValueArray *value_array);

#endif //CLOX_VALUE_H
