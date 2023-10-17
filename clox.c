#include "chunk.h"
#include "debug.h"

int main(/*int argc, const char *argv[]*/) {
    Chunk chunk;
    chunk_init(&chunk);

    chunk_push_constant(&chunk, 112.3, 1);
    chunk_push(&chunk, OP_RETURN, 1);
    chunk_push(&chunk, OP_RETURN, 2);
    chunk_push_constant(&chunk, 112.3, 2);
    chunk_push(&chunk, OP_RETURN, 2);
    chunk_push(&chunk, OP_RETURN, 2);
    chunk_push_constant(&chunk, 112.3, 3);
    chunk_push(&chunk, OP_RETURN, 3);
    chunk_push(&chunk, OP_RETURN, 3);
    chunk_push(&chunk, OP_RETURN, 3);

    disassemble_chunk(&chunk, "Test chunk");
    chunk_free(&chunk);
    return 0;
}
