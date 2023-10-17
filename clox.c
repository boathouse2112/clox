#include "common.h"
#include "chunk.h"
#include "debug.h"
#include "value.h"
#include "memory.h"

int main(/*int argc, const char *argv[]*/) {
    Chunk chunk;
    chunk_init(&chunk);

    int constant_idx = chunk_add_constant(&chunk, 1.2);
    chunk_push(&chunk, OP_CONSTANT, 1);
    chunk_push(&chunk, constant_idx, 1);
    chunk_push(&chunk, OP_RETURN, 1);
    chunk_push(&chunk, OP_CONSTANT, 2);
    chunk_push(&chunk, constant_idx, 2);
    chunk_push(&chunk, OP_RETURN, 2);
    chunk_push(&chunk, OP_RETURN, 2);
    chunk_push(&chunk, OP_RETURN, 2);
    chunk_push(&chunk, constant_idx, 3);
    chunk_push(&chunk, OP_RETURN, 3);
    chunk_push(&chunk, OP_RETURN, 3);
    chunk_push(&chunk, OP_RETURN, 3);

    disassemble_chunk(&chunk, "Test chunk");
    chunk_free(&chunk);
    return 0;
}
