#include <printf.h>
#include <stdlib.h>
#include "chunk.h"
#include "memory.h"

void line_numbers_init(LineNumbers *lines) {
    lines->length = 0;
    lines->capacity = 0;
    lines->data = NULL;
}

void line_numbers_free(LineNumbers *lines) {
    FREE_ARRAY(LineNumber, lines->data, lines->capacity);
}

int line_numbers_get(LineNumbers *lines, int target_bytecode_idx) {
    int bytes_seen = 0;
    for (int line_idx = 0; line_idx < lines->length; line_idx++) {
        LineNumber line_number = lines->data[line_idx];
        bytes_seen += line_number.run_length;
        if (target_bytecode_idx < bytes_seen) {
            return line_number.line;
        }
    }
    fprintf(stderr, "Bytecode index %d does not have a matching line number", target_bytecode_idx);
    exit(1);
}

void line_numbers_push(LineNumbers *lines, int line) {
    // Grow the array even if we don't add a node.
    // Lets us check last_line if lines is empty.
    if (lines->capacity < lines->length + 1) {
        int old_capacity = lines->capacity;
        lines->capacity = grow_capacity(old_capacity);
        lines->data = GROW_ARRAY(LineNumber, lines->data, old_capacity, lines->capacity);
    }

    LineNumber* last_line_rle = lines->data + (lines->length - 1);
    int last_line = last_line_rle->line;
    if (line == last_line) {
        // Increment run-length
        *last_line_rle = (LineNumber) {
                .line = last_line,
                .run_length = last_line_rle->run_length + 1,
        };
    } else {
        lines->data[lines->length] = (LineNumber) {
            .line = line,
            .run_length = 1,
        };
        lines->length += 1;
    }
}

/// Initialize a chunk ArrayList to 0 elements.
void chunk_init(Chunk *chunk) {
    chunk->length = 0;
    chunk->capacity = 0;
    chunk->bytecode = NULL;
    line_numbers_init(&chunk->lines);
    value_array_init(&chunk->constants);
}

void chunk_free(Chunk *chunk) {
    FREE_ARRAY(uint8_t, chunk->bytecode, chunk->capacity);
    line_numbers_free(&chunk->lines);
    value_array_free(&chunk->constants);
    chunk_init(chunk);
}

/// Push a bytecode op onto the end of the chunk.
/// Also pushes a line number to lines
/// Grows the ArrayList if necessary.
void chunk_push(Chunk *chunk, uint8_t byte, int line) {
    if (chunk->capacity < chunk->length + 1) {
        int old_capacity = chunk->capacity;
        chunk->capacity = grow_capacity(old_capacity);
        chunk->bytecode = GROW_ARRAY(uint8_t, chunk->bytecode, old_capacity, chunk->capacity);
    }

    chunk->bytecode[chunk->length] = byte;
    line_numbers_push(&chunk->lines, line);
    chunk->length += 1;
}

/// Add a constant to the line pool
/// @return the index where the constant was added
int chunk_add_constant(Chunk *chunk, Value value) {
    value_array_push(&chunk->constants, value);
    return chunk->constants.length - 1;
}
















