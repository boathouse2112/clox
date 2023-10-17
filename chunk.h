#ifndef CHUNK_H_
#define CHUNK_H_

#include "common.h"
#include "value.h"

typedef enum {
    OP_CONSTANT,
    OP_CONSTANT_LONG, // Big-endian ordering

    // ==== Arithmetic ====
    OP_NEGATE,
    OP_ADD,
    OP_SUBTRACT,
    OP_MULTIPLY,
    OP_DIVIDE,
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
int line_numbers_get(LineNumbers *lines, int bytecode_idx);
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

// TODO -- Test pushing a long constant

int chunk_get_constant_idx(Chunk *chunk, int bytecode_idx);
/// Given the index of a constant-load instruction, reads and returns that constant.
Value chunk_get_constant(Chunk *chunk, int bytecode_idx);
/// Add the given constant to the constant pool
/// Pushes an appropriate constant-load instruction to chunk->bytecode
int chunk_push_constant(Chunk *chunk, Value value, int line);

#endif // CHUNK_H_
