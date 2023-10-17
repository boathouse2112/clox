#include "chunk.h"

#ifndef CLOX_DEBUG_H
#define CLOX_DEBUG_H

void disassemble_chunk(Chunk *chunk, const char *name);
int disassemble_instruction(Chunk *chunk, int offset);

#endif //CLOX_DEBUG_H
