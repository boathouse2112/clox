#ifndef CLOX_VM_H
#define CLOX_VM_H

#include "chunk.h"

#define STACK_MAX 256

typedef enum {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR,
} InterpretResult;

typedef struct {
    Chunk *chunk;
    uint8_t *ip; // Points to next instr
    Value stack[STACK_MAX];
    Value *stack_top; // Points to first empty stack value
} Vm;

void vm_init();
void vm_free();

void vm_stack_push(Value value);
Value vm_stack_pop();

InterpretResult vm_interpret(Chunk *chunk);

#undef STACK_MAX

#endif //CLOX_VM_H
