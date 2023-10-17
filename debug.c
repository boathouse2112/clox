#include <printf.h>
#include "debug.h"

// ==== Helpers ====

static int instruction_simple(const char *name, int offset) {
    printf("%s\n", name);
    return offset + 1;
}

static int instruction_constant(const char *name, Chunk *chunk, int offset) {
    uint8_t constant = chunk->bytecode[offset + 1];
    printf("%-16s [%4d] ", name, constant);
    value_print(chunk->constants.data[constant]);
    printf("\n");
    return offset + 2;
}

static int instruction_constant_long(const char *name, Chunk *chunk, int offset) {
    uint8_t constant = chunk->bytecode[offset + 1];
    printf("%-16s [%4d] ", name, constant);
    value_print(chunk->constants.data[constant]);
    printf("\n");
    return offset + 2;
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

        case OP_RETURN:
            return instruction_simple("OP_RETURN", offset);
        default:
            printf("Unknown opcode %d\n", instr);
            return offset + 1;
    }
}
