#ifndef CLOX_COMPILER_H
#define CLOX_COMPILER_H

#include "scanner.h"
#include "chunk.h"

bool compiler_compile(const char *source, Chunk *chunk);

#endif //CLOX_COMPILER_H
