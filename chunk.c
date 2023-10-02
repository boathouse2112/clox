#include <stdlib.h>
#include "chunk.h"

void chunk_init(Chunk *chunk) {
    chunk->count = 0;
    chunk->capacity = 0;
    chunk->code = NULL;
}

//void chunk_write();