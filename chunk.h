#ifndef CHUNK_H_
#define CHUNK_H_

#include "common.h"
#include "value.h"

typedef enum {
    OP_CONSTANT,
    OP_CONSTANT_LONG,
    OP_RETURN,
} OpCode;

/// Line number encoded with run-length encoding.
typedef struct {
    int line;
    int run_length;
} LineNumber;

/// Line numbers encoded with run-length encoding.
typedef struct {
    int length;
    int capacity;
    LineNumber* data;
} LineNumbers;

void line_numbers_init(LineNumbers *lines);
void line_numbers_free(LineNumbers *lines);
/// Get the line number of the nth bytecode
int line_numbers_get(LineNumbers *lines, int idx);
void line_numbers_push(LineNumbers *lines, int line);

/// ArrayList containing bytecode instructions
typedef struct {
    int length;
    int capacity;
    uint8_t *bytecode;
    LineNumbers lines;
    ValueArray constants;
} Chunk;

void chunk_init(Chunk *chunk);
void chunk_free(Chunk *chunk);

void chunk_push(Chunk *chunk, uint8_t byte, int line);
int chunk_add_constant(Chunk *chunk, Value value);

#endif // CHUNK_H_
