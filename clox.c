#include "chunk.h"
#include "debug.h"
#include "vm.h"

int main(/*int argc, const char *argv[]*/) {
    vm_init();
    Chunk chunk;
    chunk_init(&chunk);

    chunk_push_constant(&chunk, 1.2, 5);
    chunk_push_constant(&chunk, 3.4, 1);
    chunk_push(&chunk, OP_ADD, 1);
    chunk_push_constant(&chunk, 5.6, 5);
    chunk_push(&chunk, OP_DIVIDE, 1);
    chunk_push(&chunk, OP_NEGATE, 1);
    chunk_push(&chunk, OP_RETURN, 2);

//    chunk_push_constant(&chunk, 112.3, 1);
//    chunk_push(&chunk, OP_RETURN, 1);
//    chunk_push(&chunk, OP_RETURN, 2);
//    chunk_push_constant(&chunk, 112.3, 2);
//    chunk_push(&chunk, OP_RETURN, 2);
//    chunk_push(&chunk, OP_RETURN, 2);
//    chunk_push_constant(&chunk, 112.3, 3);
//    chunk_push(&chunk, OP_RETURN, 3);
//    chunk_push(&chunk, OP_RETURN, 3);
//    chunk_push(&chunk, OP_RETURN, 3);
//
    vm_interpret(&chunk);

    chunk_free(&chunk);
    vm_free();
    return 0;
}
