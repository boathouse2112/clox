#include <printf.h>
#include <stdlib.h>
#include "debug.h"

// ==== Helpers ====

static int instruction_simple(const char *name, int offset) {
    printf("%s\n", name);
    return offset + 1;
}

static int instruction_constant(const char *name, Chunk *chunk, int offset) {
    OpCode opcode = chunk->bytecode[offset];

    int constant_idx = chunk_get_constant_idx(chunk, offset);
    printf("%-16s [%4d] ", name, constant_idx);
    Value constant = chunk_get_constant(chunk, offset);
    value_print(constant);
    printf("\n");

    switch (opcode) {
        case OP_CONSTANT: return offset + 2;
        case OP_CONSTANT_LONG: return offset + 4;
        default:
            fprintf(stderr, "instruction_constant: Opcode %#X is not a constant opcode", opcode);
            exit(1);
    }
}

// ==== Disassemble ====

/// Print disassembly for a bytecode chunk
void disassemble_chunk(Chunk *chunk, const char *name) {
    printf("==== %s ====\n", name);

    int offset = 0;
    while (offset < chunk->length) {
        offset = disassemble_instruction(chunk, offset);
    }
}

/// Print disassembly for the instruction at offset
/// @return The offset after reading this instruction
int disassemble_instruction(Chunk *chunk, int offset) {
    printf("%04X ", offset);
    int line_number = line_numbers_get(&chunk->lines, offset);
    if (offset > 0
        && line_number == line_numbers_get(&chunk->lines, offset - 1)) {
        // Multiple bytecode on same line -> print continuation
        printf("   | ");
    } else {
        printf("%4d ", line_number);
    }

    uint8_t instr = chunk->bytecode[offset];
    switch (instr) {
        case OP_CONSTANT:
            return instruction_constant("OP_CONSTANT", chunk, offset);
        case OP_CONSTANT_LONG:
            return instruction_constant("OP_CONSTANT_LONG", chunk, offset);
        case OP_RETURN:
            return instruction_simple("OP_RETURN", offset);
        default:
            printf("Unknown opcode %d\n", instr);
            return offset + 1;
    }
}
