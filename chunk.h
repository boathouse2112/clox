#ifndef CHUNK_H_
#define CHUNK_H_

#include "common.h"

typedef enum {
    OP_RETURN,
} OpCode;

/// Dynamic array containing bytecode instructions
typedef struct {
    int count;
    int capacity;
    uint8_t *code;
} Chunk;

void chunk_init(Chunk *chunk);
void chunk_write(Chunk *chunk, uint8_t byte);

#endif // CHUNK_H_
